use std::sync::Arc;

use luneth::common::UploadImageDto;
use luneth_db::DbOperator;
use serde::Serialize;
use tauri::{AppHandle, Emitter as _};

use crate::{
    common::{new_crawler, new_postman},
    handlers::TaskType,
    AppError,
};

impl super::Task {
    pub async fn new_idol(app_handle: AppHandle, db: Arc<DbOperator>) -> Self {
        log::debug!("Creating new idol scraping task ");
        let task_type = TaskType::Idol;
        log::debug!("Idol scraping task created successfully");
        Self {
            db,
            task_type,
            app_handle,
        }
    }

    pub(super) async fn crawl_idol(&self) -> Result<(), AppError> {
        log::debug!("Executing idol crawl task");

        let crawler = new_crawler().await?.start().await?;
        let mut client = new_postman().await?;

        let idol_without_image = client.get_idol_without_image().await.map_err(|e| {
            let error_msg = format!("Failed to get idols without images: {e}");
            report_idol_crawl_failed(&self.app_handle, error_msg.clone());
            AppError::SendRequestFailed(error_msg)
        })?;

        let total_count = idol_without_image.len();
        log::info!("Starting idol crawl for {total_count} idols");

        // Report start event
        report_idol_crawl_start(&self.app_handle, total_count);

        let idol_links = idol_without_image
            .iter()
            .map(|idol| (idol.id, idol.link.as_str()))
            .collect::<Vec<(i64, &str)>>();

        let mut error = String::new();
        let mut success_count = 0;
        let mut processed_count = 0;

        for (id, link) in idol_links {
            processed_count += 1;

            let image = crawler.crawl_idol_image(link).await;

            match image {
                Ok(image) => {
                    // block page
                    if image.mime.contains("html") {
                        log::error!("Crawled HTML content for idol {}", image.name);
                        let error_msg = format!("Crawled HTML content for idol {}", image.name);
                        error.push_str(&format!("{error_msg}\n"));
                        report_idol_crawl_progress(&self.app_handle, processed_count, error_msg);
                        continue;
                    }

                    let upload = UploadImageDto {
                        id: id.to_string(),
                        images: vec![image],
                    };

                    let upload_result = client.post_idol_image(upload).await;
                    match upload_result {
                        Ok(resp) => {
                            success_count += 1;
                            log::info!("Image uploaded successfully for idol {id}: {resp}");
                            report_idol_crawl_progress(
                                &self.app_handle,
                                processed_count,
                                format!("Successfully processed idol {id}"),
                            );
                        }
                        Err(e) => {
                            log::error!("Failed to upload image for idol {id}: {e}");
                            let error_msg = format!("Failed to upload image for idol {id}: {e}");
                            error.push_str(&format!("{error_msg}\n"));
                            report_idol_crawl_progress(
                                &self.app_handle,
                                processed_count,
                                error_msg,
                            );
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to crawl image: {e}");
                    let error_msg = format!("Failed to crawl image: {e}");
                    error.push_str(&format!("{error_msg}\n"));
                    report_idol_crawl_progress(&self.app_handle, processed_count, error_msg);
                }
            }
        }

        if error.is_empty() {
            log::info!(
                "Idol crawl completed successfully: {success_count}/{total_count} processed"
            );
            report_idol_crawl_complete(&self.app_handle, success_count, total_count);
            Ok(())
        } else {
            let final_error = format!(
                "Idol crawl completed with errors: {success_count}/{total_count} successful. Errors:\n{error}"
            );
            log::error!("{final_error}");
            report_idol_crawl_failed(&self.app_handle, final_error.clone());
            Err(AppError::SendRequestFailed(final_error))
        }
    }
}

// ################
// # Report Events
// ###############

// Idol crawl events
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct IdolCrawlStartEvent {
    total_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct IdolCrawlProgressEvent {
    processed: usize,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct IdolCrawlCompleteEvent {
    success_count: usize,
    total_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct IdolCrawlFailedEvent {
    error_message: String,
}

// Event emission helper functions
fn report_idol_crawl_start(app_handle: &AppHandle, total_count: usize) {
    let event = IdolCrawlStartEvent { total_count };
    match app_handle.emit("idol-crawl-start", &event) {
        Ok(_) => log::debug!("Emitted idol-crawl-start event with {total_count} idols"),
        Err(e) => log::error!("Failed to emit idol-crawl-start event: {e}"),
    }
}

fn report_idol_crawl_progress(app_handle: &AppHandle, processed: usize, message: String) {
    let event = IdolCrawlProgressEvent { processed, message };
    match app_handle.emit("idol-crawl-progress", &event) {
        Ok(_) => log::debug!("Emitted idol-crawl-progress event: {processed} processed"),
        Err(e) => log::error!("Failed to emit idol-crawl-progress event: {e}"),
    }
}

fn report_idol_crawl_complete(app_handle: &AppHandle, success_count: usize, total_count: usize) {
    let event = IdolCrawlCompleteEvent {
        success_count,
        total_count,
    };
    match app_handle.emit("idol-crawl-complete", &event) {
        Ok(_) => log::debug!(
            "Emitted idol-crawl-complete event: {success_count}/{total_count} successful"
        ),
        Err(e) => log::error!("Failed to emit idol-crawl-complete event: {e}"),
    }
}

fn report_idol_crawl_failed(app_handle: &AppHandle, error_message: String) {
    let event = IdolCrawlFailedEvent { error_message };
    match app_handle.emit("idol-crawl-failed", &event) {
        Ok(_) => log::debug!("Emitted idol-crawl-failed event"),
        Err(e) => log::error!("Failed to emit idol-crawl-failed event: {e}"),
    }
}
