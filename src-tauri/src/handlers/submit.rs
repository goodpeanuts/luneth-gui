use std::sync::Arc;

use luneth::common::UploadImageDto;
use luneth_db::{DbOperator, DbService as _, OperationType};
use serde::Serialize;
use tauri::{AppHandle, Emitter as _};

use crate::{
    common::new_postman,
    db::{
        log::{log_failed_op, log_success_op},
        write::mark_record_submitted,
    },
    handlers::{images, TaskType},
    AppError,
};

impl super::Task {
    pub async fn new_submit(
        app_handle: AppHandle,
        db: Arc<DbOperator>,
        codes: Vec<String>,
    ) -> Result<Self, AppError> {
        let task_type = TaskType::Submit(codes);
        Ok(Self {
            db,
            task_type,
            app_handle,
        })
    }

    #[expect(clippy::too_many_lines)]
    pub(super) async fn submit_codes(&self, codes: &[String]) -> Result<(), AppError> {
        use luneth_db::record_local::Entity as RecordEntity;
        let mut client = new_postman().await?;

        let mut error_count = 0;
        let mut success_count = 0;
        let total_count = codes.len();

        // Send initial progress event
        report_submit_start(&self.app_handle, total_count);

        for code in codes {
            log::debug!("Processing submit for code: {code}");

            // Send individual code start event
            report_submit_code_start(&self.app_handle, code);

            let Some(local_record) = self
                .db
                .find_record_local_by_id::<RecordEntity>(code)
                .await?
            else {
                let error_msg = format!("Failed to find record for code: {code}");
                log::error!("{error_msg}");
                log_failed_op(
                    self.db.as_ref(),
                    OperationType::Submit,
                    code,
                    error_msg.clone(),
                )
                .await?;

                error_count += 1;
                report_submit_code_result(&self.app_handle, code, SubmitStatus::Failed, error_msg);
                continue;
            };

            let record = local_record.into_record();
            let images =
                match images::read_local_images(&self.app_handle, code, record.local_image_count)
                    .await
                {
                    Ok(images) => {
                        log::debug!("Read {} images for record {code}", images.len());
                        if record.local_image_count != images.len() as i32 {
                            let error_msg = format!(
                                "Image count mismatch for code: {}. Expected {}, found {}",
                                code,
                                record.local_image_count,
                                images.len()
                            );
                            log::error!("{error_msg}");
                            log_failed_op(
                                self.db.as_ref(),
                                OperationType::Submit,
                                code,
                                error_msg.clone(),
                            )
                            .await?;

                            error_count += 1;
                            report_submit_code_result(
                                &self.app_handle,
                                code,
                                SubmitStatus::Failed,
                                error_msg,
                            );
                            continue;
                        }
                        images
                    }
                    Err(e) => {
                        let error_msg =
                            format!("Failed to read local images for code: {code}. Error: {e}");
                        log::error!("{error_msg}");
                        log_failed_op(
                            self.db.as_ref(),
                            OperationType::Submit,
                            code,
                            error_msg.clone(),
                        )
                        .await?;

                        error_count += 1;
                        report_submit_code_result(
                            &self.app_handle,
                            code,
                            SubmitStatus::Failed,
                            error_msg,
                        );
                        continue;
                    }
                };

            let upload_images = UploadImageDto {
                id: record.id.clone(),
                images,
            };

            let post_record_resp = client.post_single_record(&record).await;
            let post_images_resp = client.post_image(upload_images).await;

            match (post_record_resp, post_images_resp) {
                (Ok(_), Ok(_)) => {
                    success_count += 1;
                    mark_record_submitted(self.db.as_ref(), record.id.as_str()).await?;
                    log_success_op(self.db.as_ref(), OperationType::Submit, code).await?;
                    log::info!("Successfully submitted record for code: {code}");

                    report_submit_code_result(
                        &self.app_handle,
                        code,
                        SubmitStatus::Success,
                        "Successfully submitted".to_owned(),
                    );
                }
                (Err(e1), Err(e2)) => {
                    let error_msg = format!(
                        "Failed to submit record and images for code: {code}. Errors: {e1}, {e2}"
                    );
                    log::error!("{error_msg}");
                    log_failed_op(
                        self.db.as_ref(),
                        OperationType::Submit,
                        code,
                        error_msg.clone(),
                    )
                    .await?;

                    error_count += 1;
                    report_submit_code_result(
                        &self.app_handle,
                        code,
                        SubmitStatus::Failed,
                        error_msg,
                    );
                }
                (Err(err), _) => {
                    let error_msg =
                        format!("Failed to submit record for code: {code}. Error: {err}");
                    log::error!("{error_msg}");
                    log_failed_op(
                        self.db.as_ref(),
                        OperationType::Submit,
                        code,
                        error_msg.clone(),
                    )
                    .await?;

                    error_count += 1;
                    report_submit_code_result(
                        &self.app_handle,
                        code,
                        SubmitStatus::Failed,
                        error_msg,
                    );
                }
                (_, Err(err)) => {
                    let error_msg =
                        format!("Failed to submit images for code: {code}. Error: {err}");
                    log::error!("{error_msg}");
                    log_failed_op(
                        self.db.as_ref(),
                        OperationType::Submit,
                        code,
                        error_msg.clone(),
                    )
                    .await?;

                    error_count += 1;
                    report_submit_code_result(
                        &self.app_handle,
                        code,
                        SubmitStatus::Failed,
                        error_msg,
                    );
                }
            }
        }

        // Send finished event
        report_submit_finished(&self.app_handle, success_count, error_count, total_count);

        log::info!(
            "Submit task completed: {success_count} successful, {error_count} errors out of {total_count} total codes"
        );

        Ok(())
    }
}

// ################
// # Report Events
// ###############

// Progress status for submit operations
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
enum SubmitStatus {
    Success,
    Failed,
}

// Progress events for submit operations
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubmitStartEvent {
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubmitCodeStartEvent {
    pub code: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubmitCodeResultEvent {
    pub code: String,
    pub status: SubmitStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubmitFinishedEvent {
    pub success_count: usize,
    pub error_count: usize,
    pub total_count: usize,
}

// Event emission helper functions
fn report_submit_start(app_handle: &AppHandle, total_count: usize) {
    let event = SubmitStartEvent { total_count };
    match app_handle.emit("submit-start", &event) {
        Ok(_) => log::debug!("Emitted submit-start event with {total_count} codes"),
        Err(e) => log::error!("Failed to emit submit-start event: {e}"),
    }
}

fn report_submit_code_start(app_handle: &AppHandle, code: &str) {
    let event = SubmitCodeStartEvent {
        code: code.to_owned(),
    };
    match app_handle.emit("submit-code-start", &event) {
        Ok(_) => log::debug!("Emitted submit-code-start event for {code}"),
        Err(e) => log::error!("Failed to emit submit-code-start event: {e}"),
    }
}

fn report_submit_code_result(
    app_handle: &AppHandle,
    code: &str,
    status: SubmitStatus,
    message: String,
) {
    let event = SubmitCodeResultEvent {
        code: code.to_owned(),
        status,
        message,
    };
    match app_handle.emit("submit-code-result", &event) {
        Ok(_) => log::debug!("Emitted submit-code-result event for {code}: {status:?}"),
        Err(e) => log::error!("Failed to emit submit-code-result event: {e}"),
    }
}

fn report_submit_finished(
    app_handle: &AppHandle,
    success_count: usize,
    error_count: usize,
    total_count: usize,
) {
    let event = SubmitFinishedEvent {
        success_count,
        error_count,
        total_count,
    };
    match app_handle.emit("submit-finished", &event) {
        Ok(_) => {
            log::debug!("Emitted submit-finished event: {success_count}/{total_count} successful");
        }
        Err(e) => log::error!("Failed to emit submit-finished event: {e}"),
    }
}
