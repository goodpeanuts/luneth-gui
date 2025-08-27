use std::{ops::Not as _, sync::Arc};

use luneth::crawl::CrawlInput;
use luneth_db::{DbOperator, DbService as _};
use serde::Serialize;
use tauri::{AppHandle, Emitter as _};

use crate::{
    common::new_crawler_with_config,
    db::log::{log_failed_op, log_success_op},
    handlers::{images::crawl_record_image, BatchCrawlConfig, TaskType},
    AppError,
};
use luneth_db::OperationType;

impl super::Task {
    pub async fn new_update(
        app_handle: AppHandle,
        db: Arc<DbOperator>,
        config: BatchCrawlConfig,
    ) -> Result<Self, AppError> {
        let task_type = TaskType::Update(config);
        Ok(Self {
            app_handle,
            db,
            task_type,
        })
    }

    #[expect(clippy::too_many_lines)]
    pub async fn update_codes(&self, config: &BatchCrawlConfig) -> Result<(), AppError> {
        let batch = &config.batch;
        let config = config.crawl_config.clone();
        log::debug!("Executing update crawl task for {} codes", batch.len());

        let crawler = new_crawler_with_config(config).await?.start().await?;
        let mut update_count = 0;
        let mut success_count = 0;
        let mut error_count = 0;

        let inputs = batch
            .iter()
            .map(|code| CrawlInput::Code(code.to_owned()))
            .collect::<Vec<_>>();
        let total_count = inputs.len();

        // Send initial progress event
        report_update_start(&self.app_handle, total_count);

        for input in inputs {
            let code = input.get_code().to_owned();
            match crawler.crawl_recorder(input).await {
                Ok(recorder) => {
                    use luneth_db::record_local::Entity as RecordEntity;
                    let Some(local_record) = self
                        .db
                        .find_record_local_by_id::<RecordEntity>(&code)
                        .await?
                    else {
                        log::error!("Record {code} not found in local database, skipping update");
                        log_failed_op(
                            self.db.as_ref(),
                            OperationType::Update,
                            &code,
                            "Record not found".to_owned(),
                        )
                        .await?;
                        error_count += 1;

                        report_update_code_result(
                            &self.app_handle,
                            &code,
                            UpdateStatus::Failed,
                            "Record not found in local database".to_owned(),
                        );
                        continue;
                    };

                    let mut updated_something = false;
                    let mut update_messages = Vec::new();

                    if local_record.is_cached_locally.not() {
                        if let Err(e) =
                            crawl_record_image(&self.app_handle, &crawler, &recorder).await
                        {
                            log::error!("Failed to crawl images for record {code}: {e}");
                            log_failed_op(
                                self.db.as_ref(),
                                OperationType::Update,
                                &code,
                                format!("Failed to crawl images: {e}"),
                            )
                            .await?;
                            error_count += 1;

                            report_update_code_result(
                                &self.app_handle,
                                &code,
                                UpdateStatus::Failed,
                                format!("Failed to crawl images: {e}"),
                            );
                            continue;
                        } else {
                            let updated_model = local_record
                                .clone()
                                .set_local_image_cached(recorder.record.local_image_count, true);
                            self.db.update_record_local(updated_model).await?;
                            log::info!("Successfully crawled images for record {code} by update");
                            updated_something = true;
                            update_messages.push("crawled images".to_owned());
                        }
                    }

                    let local_record_entry = local_record.into_record();
                    if local_record_entry.share_magnet_links.len()
                        < recorder.record.share_magnet_links.len()
                    {
                        let updated_model =
                            local_record.update_links(recorder.record.share_magnet_links.clone());
                        self.db.update_record_local(updated_model).await?;
                        log::info!("Updated share links for record {code}");
                        updated_something = true;
                        update_messages.push("updated magnet links".to_owned());
                    }

                    if updated_something {
                        update_count += 1;
                        success_count += 1;
                        log_success_op(self.db.as_ref(), OperationType::Update, &code).await?;
                        log::debug!("Successfully updated {code}");

                        report_update_code_result(
                            &self.app_handle,
                            &code,
                            UpdateStatus::Success,
                            format!("Updated: {}", update_messages.join(", ")),
                        );
                    } else {
                        success_count += 1;
                        report_update_code_result(
                            &self.app_handle,
                            &code,
                            UpdateStatus::Success,
                            "No updates needed".to_owned(),
                        );
                    }
                }
                Err(e) => {
                    log::error!("Failed crawl {code} to update: {e}");
                    log_failed_op(
                        self.db.as_ref(),
                        OperationType::Update,
                        &code,
                        e.to_string(),
                    )
                    .await?;
                    error_count += 1;

                    report_update_code_result(
                        &self.app_handle,
                        &code,
                        UpdateStatus::Failed,
                        format!("Crawl failed: {e}"),
                    );
                }
            }
        }

        log::info!("Updated {update_count} codes");

        // Send finished event
        report_update_finished(&self.app_handle, success_count, error_count, total_count);

        Ok(())
    }
}

// ################
// # Report Events
// ###############

// Progress status for update operations
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
enum UpdateStatus {
    Success,
    Failed,
}

// Progress events for update operations
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateCodeReportEvent {
    pub code: String,
    pub status: UpdateStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateFinishedEvent {
    pub success_count: usize,
    pub error_count: usize,
    pub total_count: usize,
}

// Event emission helper functions
fn report_update_start(app_handle: &AppHandle, total_count: usize) {
    let event = serde_json::json!({
        "totalCount": total_count
    });
    match app_handle.emit("update-start", &event) {
        Ok(_) => log::debug!("Emitted update-start event with {total_count} codes"),
        Err(e) => log::error!("Failed to emit update-start event: {e}"),
    }
}

fn report_update_code_result(
    app_handle: &AppHandle,
    code: &str,
    status: UpdateStatus,
    message: String,
) {
    let event = UpdateCodeReportEvent {
        code: code.to_owned(),
        status,
        message,
    };
    match app_handle.emit("update-code-report", &event) {
        Ok(_) => log::debug!("Emitted update-code-report event for {code}: {status:?}"),
        Err(e) => log::error!("Failed to emit update-code-report event: {e}"),
    }
}

fn report_update_finished(
    app_handle: &AppHandle,
    success_count: usize,
    error_count: usize,
    total_count: usize,
) {
    let event = UpdateFinishedEvent {
        success_count,
        error_count,
        total_count,
    };
    match app_handle.emit("update-finished", &event) {
        Ok(_) => {
            log::debug!("Emitted update-finished event: {success_count}/{total_count} successful");
        }
        Err(e) => log::error!("Failed to emit update-finished event: {e}"),
    }
}
