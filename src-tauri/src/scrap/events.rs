use serde::Serialize;
use tauri::{AppHandle, Emitter as _};

// Progress status for crawl operations
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CrawlStatus {
    Success,
    Failed,
}

// Progress events for page crawling (auto mode)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlPageStartEvent {
    pub page_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlPageSuccessEvent {
    pub page_name: String,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlPageFailedEvent {
    pub page_name: String,
    pub error_message: String,
}

// Progress events for code crawling
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlCodeReportEvent {
    pub code: String,
    pub status: CrawlStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlCodesFinishedEvent {
    pub success_count: usize,
    pub error_count: usize,
    pub total_count: usize,
}

// Event emission helper functions
pub fn report_crawl_manual_start(app_handle: &AppHandle, total_count: usize) {
    let event = serde_json::json!({
        "totalCount": total_count
    });
    match app_handle.emit("crawl-manual-start", &event) {
        Ok(_) => log::debug!("Emitted crawl-manual-start event with {total_count} codes"),
        Err(e) => log::error!("Failed to emit crawl-manual-start event: {e}"),
    }
}

pub fn report_crawl_page_start(app_handle: &AppHandle, page_name: &str) {
    let event = CrawlPageStartEvent {
        page_name: page_name.to_owned(),
    };
    match app_handle.emit("crawl-page-start", &event) {
        Ok(_) => log::debug!("Emitted crawl-page-start event for {page_name}"),
        Err(e) => log::error!("Failed to emit crawl-page-start event: {e}"),
    }
}

pub fn report_crawl_page_success(app_handle: &AppHandle, page_name: &str, total_count: usize) {
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

pub fn report_crawl_page_failed(app_handle: &AppHandle, page_name: &str, error_message: String) {
    let event = CrawlPageFailedEvent {
        page_name: page_name.to_owned(),
        error_message,
    };
    match app_handle.emit("crawl-page-failed", &event) {
        Ok(_) => log::debug!("Emitted crawl-page-failed event for {page_name}"),
        Err(e) => log::error!("Failed to emit crawl-page-failed event: {e}"),
    }
}

pub fn report_crawl_code_result(
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

pub fn report_crawl_codes_finished(
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
