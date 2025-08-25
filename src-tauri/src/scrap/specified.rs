use std::fmt::Debug;
use std::fs;

use luneth::crawl::WebCrawler;
use tauri::AppHandle;

use super::events::{
    report_crawl_code_result, report_crawl_codes_finished, report_crawl_manual_start, CrawlStatus,
};
use crate::db::{log_failed_crawl_record_op, log_success_crawl_record_op};
use crate::scrap::images::crawl_record_image;
use crate::AppError;
use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::DbService;

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

    // Send initial progress event for manual mode
    report_crawl_manual_start(app_handle, codes.len());

    for code in codes {
        let code = code.as_ref();
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
                        log_success_crawl_record_op(db, code).await?;
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
                        log_failed_crawl_record_op(db, code, e.to_string()).await?;
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
                log_failed_crawl_record_op(db, code, e.to_string()).await?;
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
