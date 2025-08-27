use std::fmt::Debug;
use std::fs;
use std::sync::Arc;

use luneth::crawl::WebCrawler;
use serde::Serialize;
use tauri::{AppHandle, Emitter as _};

use crate::common::{new_crawler, EXIST_IDS};
use crate::db::log::{log_failed_op, log_success_op};
use crate::handlers::images::crawl_record_image;
use crate::handlers::TaskType;
use crate::AppError;
use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::{DbOperator, DbService, OperationType};

impl super::Task {
    pub async fn new_manual(
        app_handle: AppHandle,
        db: Arc<DbOperator>,
        codes: Vec<String>,
        with_image: bool,
    ) -> Result<Self, AppError> {
        log::debug!(
            "Creating new manual scraping task for {} codes",
            codes.len()
        );
        let task_type = TaskType::Batch(codes, with_image);
        log::debug!("Manual scraping task created successfully");
        Ok(Self {
            db,
            task_type,
            app_handle,
        })
    }

    pub(super) async fn crawl_batch(
        &self,
        codes: &[String],
        with_image: bool,
    ) -> Result<(), AppError> {
        let crawler = new_crawler().await?.start().await?;

        crawl_codes(
            &self.app_handle,
            self.db.as_ref(),
            &crawler,
            codes,
            with_image,
        )
        .await
    }
}

pub async fn crawl_codes<T>(
    app_handle: &AppHandle,
    db: &impl DbService,
    crawler: &WebCrawler,
    codes: &[T],
    with_image: bool,
) -> Result<(), AppError>
where
    T: AsRef<str> + Debug,
{
    log::debug!("Starting to crawl {} codes", codes.len());
    let mut success_count = 0;
    let mut error_count = 0;

    EXIST_IDS.write().await.fresh(db).await;
    let exist_records = EXIST_IDS.read().await.ids.clone();

    // Send initial progress event - unified batch crawl start
    report_batch_crawl_start(app_handle, codes.len());

    for code in codes {
        let code = code.as_ref();
        if exist_records.contains(&code.to_owned()) {
            log::debug!("skip, {code} already exist");
            report_crawl_code_result(
                app_handle,
                code,
                CrawlStatus::Exist,
                "Record already exists".to_owned(),
            );
            success_count += 1;
            continue;
        }

        log::debug!("Crawling code: {code}");

        match crawler.crawl_code(code).await {
            Ok(record) => {
                let mut image_path_dir = None;
                let record_model = if with_image {
                    image_path_dir = crawl_record_image(app_handle, crawler, &record).await.ok();
                    RecorderModel::from_recorder_with_image_local(&record)
                } else {
                    RecorderModel::from_recorder(&record)
                };

                let insert_result = db.insert_entity(record_model).await;

                match insert_result {
                    Ok(_) => {
                        // crawl_code_report
                        // log_success_crawl_record_op(db, code).await?;
                        log_success_op(db, OperationType::CrawlRecord, code).await?;
                        success_count += 1;
                        log::info!("Successfully crawled and saved code: {code}");

                        // Send progress event to frontend
                        report_crawl_code_result(
                            app_handle,
                            code,
                            CrawlStatus::Success,
                            "Successfully crawled".to_owned(),
                        );
                    }
                    Err(e) => {
                        // crawl_code_report
                        if let Some(image_path_dir) = image_path_dir {
                            fs::remove_dir_all(image_path_dir).map_err(AppError::from)?;
                        }
                        log::error!("Failed to insert record for code {code}: {e}");
                        log_failed_op(db, OperationType::CrawlRecord, code, e.to_string()).await?;
                        error_count += 1;

                        // Send progress event to frontend
                        report_crawl_code_result(
                            app_handle,
                            code,
                            CrawlStatus::Failed,
                            format!("Failed to save: {e}"),
                        );
                    }
                };
            }
            Err(e) => {
                // crawl_code_report
                log::warn!("Failed to crawl code {code}: {e}");
                log_failed_op(db, OperationType::CrawlRecord, code, e.to_string()).await?;
                error_count += 1;

                // Send progress event to frontend
                report_crawl_code_result(
                    app_handle,
                    code,
                    CrawlStatus::Failed,
                    format!("Crawl failed: {e}"),
                );
            }
        }
    }

    // crawl_codes_finished
    log::info!(
        "Crawling completed: {success_count} successful, {error_count} errors out of {} total codes",
        codes.len()
    );

    // Send finished event to frontend
    report_crawl_codes_finished(app_handle, success_count, error_count, codes.len());

    Ok(())
}

// ################
// # Report Events
// ###############

// Progress status for crawl operations
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
enum CrawlStatus {
    Success,
    Failed,
    Exist,
}

// Progress events for code crawling
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BatchCrawlStartEvent {
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CrawlCodeReportEvent {
    pub code: String,
    pub status: CrawlStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CrawlCodesFinishedEvent {
    pub success_count: usize,
    pub error_count: usize,
    pub total_count: usize,
}

// Event emission helper functions
fn report_batch_crawl_start(app_handle: &AppHandle, total_count: usize) {
    let event = BatchCrawlStartEvent { total_count };
    match app_handle.emit("batch-crawl-start", &event) {
        Ok(_) => log::debug!("Emitted batch-crawl-start event with {total_count} codes"),
        Err(e) => log::error!("Failed to emit batch-crawl-start event: {e}"),
    }
}

fn report_crawl_code_result(
    app_handle: &AppHandle,
    code: &str,
    status: CrawlStatus,
    message: String,
) {
    let event = CrawlCodeReportEvent {
        code: code.to_owned(),
        status,
        message,
    };
    match app_handle.emit("crawl-code-report", &event) {
        Ok(_) => log::debug!("Emitted crawl-code-report event for {code}: {status:?}"),
        Err(e) => log::error!("Failed to emit crawl-code-report event: {e}"),
    }
}

fn report_crawl_codes_finished(
    app_handle: &AppHandle,
    success_count: usize,
    error_count: usize,
    total_count: usize,
) {
    let event = CrawlCodesFinishedEvent {
        success_count,
        error_count,
        total_count,
    };
    match app_handle.emit("crawl-codes-finished", &event) {
        Ok(_) => log::debug!(
            "Emitted crawl-codes-finished event: {success_count}/{total_count} successful"
        ),
        Err(e) => log::error!("Failed to emit crawl-codes-finished event: {e}"),
    }
}
