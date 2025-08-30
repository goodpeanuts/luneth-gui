use luneth::crawl::CrawlInput;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Emitter as _};

use crate::{
    common::new_crawler_with_config,
    db::log::{log_failed_op, log_success_op},
    handlers::{AppError, AutoCrawlConfig, TaskType},
};
use luneth_db::{DbOperator, DbService, OperationType};

// XXX: conditionally stop
const MAX_ITER_DEPTH: usize = 120;

impl super::Task {
    #[expect(clippy::too_many_arguments)]
    pub async fn new_auto(
        app_handle: AppHandle,
        db: Arc<DbOperator>,
        start_url: String,
        with_image: bool,
        headless: bool,
        load_timeout: u64,
        request_delay: u64,
        webdriver_port: u16,
    ) -> Result<Self, AppError> {
        log::debug!("Creating new auto scraping task for URL: {start_url}");
        let config = AutoCrawlConfig::new(
            start_url,
            with_image,
            headless,
            load_timeout,
            request_delay,
            webdriver_port,
        )
        .await?;
        let task_type = TaskType::Auto(config);
        log::debug!("Auto scraping task created successfully");
        Ok(Self {
            db,
            task_type,
            app_handle,
        })
    }

    pub(super) async fn crawl_auto(&self, config: &AutoCrawlConfig) -> Result<(), AppError> {
        auto_crawl_page(&self.app_handle, self.db.as_ref(), config).await
    }
}

async fn auto_crawl_page(
    app_handle: &AppHandle,
    db: &impl DbService,
    config: &AutoCrawlConfig,
) -> Result<(), AppError> {
    let start_url = config.start_url.clone();
    let with_image = config.with_image;
    let config = config.crawl_config.clone();

    let crawler = new_crawler_with_config(config).await?.start().await?;

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

                log_success_op(db, OperationType::CrawlPage, &page_name).await?;

                // Send page success event to frontend
                report_crawl_page_success(app_handle, &page_name, record_pieces.len());

                record_pieces
            }
            Err(e) => {
                // crawl_page_failed(page_i, msg);
                log::error!("Failed to crawl page {url}: {e}");

                log_failed_op(db, OperationType::CrawlPage, &page_name, e.to_string()).await?;

                // Send page failed event to frontend
                report_crawl_page_failed(app_handle, &page_name, e.to_string());

                continue;
            }
        };

        let record_inputs = record_pieces
            .into_iter()
            .map(CrawlInput::Piece)
            .collect::<Vec<_>>();

        if record_inputs.is_empty() {
            log::warn!("No records found on page {i}, stopping crawl");
            break;
        }

        let total_count = record_inputs.len();

        super::batch::crawl_codes(app_handle, db, &crawler, record_inputs, with_image).await?;
        // crawl_page_finished(page_i);
        log::info!("Successfully crawled page: {url}, found {total_count} records",);

        // Send page finished event to frontend (this will be sent by crawl_codes_finished event)
    }

    log::info!("Auto crawl completed for URL: {start_url}");
    Ok(())
}

// ################
// # Report Events
// ###############

// Progress events for page crawling (auto mode)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CrawlPageStartEvent {
    pub page_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CrawlPageSuccessEvent {
    pub page_name: String,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CrawlPageFailedEvent {
    pub page_name: String,
    pub error_message: String,
}

fn report_crawl_page_start(app_handle: &AppHandle, page_name: &str) {
    let event = CrawlPageStartEvent {
        page_name: page_name.to_owned(),
    };
    match app_handle.emit("crawl-page-start", &event) {
        Ok(_) => log::debug!("Emitted crawl-page-start event for {page_name}"),
        Err(e) => log::error!("Failed to emit crawl-page-start event: {e}"),
    }
}

fn report_crawl_page_success(app_handle: &AppHandle, page_name: &str, total_count: usize) {
    let event = CrawlPageSuccessEvent {
        page_name: page_name.to_owned(),
        total_count,
    };
    match app_handle.emit("crawl-page-success", &event) {
        Ok(_) => {
            log::debug!(
                "Emitted crawl-page-success event for {page_name} with {total_count} items"
            );
        }
        Err(e) => log::error!("Failed to emit crawl-page-success event: {e}"),
    }
}

fn report_crawl_page_failed(app_handle: &AppHandle, page_name: &str, error_message: String) {
    let event = CrawlPageFailedEvent {
        page_name: page_name.to_owned(),
        error_message,
    };
    match app_handle.emit("crawl-page-failed", &event) {
        Ok(_) => log::debug!("Emitted crawl-page-failed event for {page_name}"),
        Err(e) => log::error!("Failed to emit crawl-page-failed event: {e}"),
    }
}
