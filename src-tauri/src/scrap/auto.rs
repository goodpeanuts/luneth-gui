use luneth::crawl::WebCrawler;
use tauri::AppHandle;

use crate::scrap::{
    events::{report_crawl_page_failed, report_crawl_page_start, report_crawl_page_success},
    AppError,
};
use luneth_db::DbService;

// XXX: conditionally stop
const MAX_ITER_DEPTH: usize = 30;

pub async fn auto_crawl_page(
    app_handle: &AppHandle,
    db: &impl DbService,
    crawler: &WebCrawler,
    start_url: &str,
) -> Result<(), AppError> {
    log::debug!("Starting auto crawl for URL: {start_url}");

    for i in 1..=MAX_ITER_DEPTH {
        let url = format!("{start_url}page/{i}");
        let page_name = format!("page/{i}");
        log::debug!("Crawling page: {url}");

        // Send page start event to frontend
        report_crawl_page_start(app_handle, &page_name);

        // crawl_page_start(page_i)
        let record_pieces = match crawler.crawl_page(&url).await {
            Ok(record_pieces) => {
                // crawl_page_success(page_i, len);
                log::debug!("Successfully crawled page: {url}");

                // Send page success event to frontend
                report_crawl_page_success(app_handle, &page_name, record_pieces.len());

                record_pieces
            }
            Err(e) => {
                // crawl_page_failed(page_i, msg);
                log::error!("Failed to crawl page {url}: {e}");

                // Send page failed event to frontend
                report_crawl_page_failed(app_handle, &page_name, e.to_string());

                continue;
            }
        };

        let record_ids = record_pieces
            .into_iter()
            .map(|r| r.code)
            .collect::<Vec<_>>();

        if record_ids.is_empty() {
            log::warn!("No records found on page {i}, stopping crawl");
            break;
        }

        super::specified::crawl_codes(app_handle, db, crawler, &record_ids).await?;
        // crawl_page_finished(page_i);
        log::info!(
            "Successfully crawled page: {url}, found {} records",
            record_ids.len()
        );

        // Send page finished event to frontend (this will be sent by crawl_codes_finished event)
    }

    log::info!("Auto crawl completed for URL: {start_url}");
    Ok(())
}
