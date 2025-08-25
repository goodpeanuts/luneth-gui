use std::sync::{Arc, LazyLock};

use luneth::crawl::{self, crawler};
use luneth_db::DbOperator;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::AppError;

mod auto;
mod idol;
mod images;
mod record;
mod specified;

pub static TASK_BASE_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| {
    Mutex::new(None) // Default base URL
});

#[derive(Debug)]
pub enum TaskType {
    // Start URL, and with image
    Auto(String, bool),

    // List of specified codes, and with image
    Manual(Vec<String>, bool),

    // Pull remote records
    PullRemote,

    // Idol Link
    Idol,
}

pub struct Task {
    app_handle: AppHandle,
    db: Arc<DbOperator>,
    task_type: TaskType,
    crawler: crawler::WebCrawler,
}

impl Task {
    async fn new_crawler() -> Result<crawler::WebCrawler, AppError> {
        log::debug!("Creating new web crawler");
        let Some(base_url) = TASK_BASE_URL.lock().await.clone() else {
            log::warn!("No base URL configured, using default crawler");
            return crawl::WebCrawler::new().map_err(AppError::CrawlError);
        };

        log::debug!("Creating crawler with base URL: {base_url}");
        let config = luneth::crawl::CrawlConfig {
            base_url: base_url.clone(),
            ..Default::default()
        };
        crawler::WebCrawler::with_config(config).map_err(AppError::CrawlError)
    }

    #[expect(unused)]
    pub fn get_crawler(&self) -> &crawler::WebCrawler {
        &self.crawler
    }

    pub async fn exec(mut self) -> Result<(), AppError> {
        log::debug!("Starting task execution");
        log::debug!("Starting web crawler");

        let result = match &self.task_type {
            TaskType::Auto(url, with_image) => {
                self.crawler = self.crawler.start().await?;
                log::debug!("Executing auto crawl task for URL: {url}");
                self.crawl_auto(url, *with_image).await
            }
            TaskType::Manual(codes, with_image) => {
                self.crawler = self.crawler.start().await?;
                log::debug!("Executing manual crawl task for {} codes", codes.len());
                self.crawl_manual(codes, *with_image).await
            }
            TaskType::Idol => {
                self.crawler = self.crawler.start().await?;
                log::debug!("Executing idol crawl task");
                self.crawl_idol().await
            }
            TaskType::PullRemote => self.pull_record_slim().await,
        };

        match &result {
            Ok(_) => log::info!("{:?} Task execution completed successfully", self.task_type),
            Err(e) => log::error!("Task execution failed: {e}"),
        }

        result
    }
}
