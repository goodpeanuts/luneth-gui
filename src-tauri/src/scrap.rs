use std::sync::{Arc, LazyLock};

use luneth::crawl::{self, crawler};
use luneth_db::DbOperator;
use tokio::sync::Mutex;

use crate::AppError;

mod auto;
mod specified;

#[derive(Debug)]
pub enum TaskType {
    // Start URL
    Auto(String),

    Manual(Vec<String>),
}

pub static TASK_BASE_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| {
    Mutex::new(None) // Default base URL
});

pub struct Task {
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

    pub async fn new_auto(db: Arc<DbOperator>, start_url: String) -> Result<Self, AppError> {
        log::debug!("Creating new auto scraping task for URL: {start_url}");
        let task_type = TaskType::Auto(start_url);
        let crawler = Self::new_crawler().await?;
        log::debug!("Auto scraping task created successfully");
        Ok(Self {
            db,
            task_type,
            crawler,
        })
    }

    pub async fn new_manual(db: Arc<DbOperator>, codes: Vec<String>) -> Result<Self, AppError> {
        log::debug!(
            "Creating new manual scraping task for {} codes",
            codes.len()
        );
        let task_type = TaskType::Manual(codes);
        let crawler = Self::new_crawler().await?;
        log::debug!("Manual scraping task created successfully");
        Ok(Self {
            db,
            task_type,
            crawler,
        })
    }

    #[expect(unused)]
    pub fn get_crawler(&self) -> &crawler::WebCrawler {
        &self.crawler
    }

    pub async fn exec(mut self) -> Result<(), AppError> {
        log::debug!("Starting task execution");
        log::debug!("Starting web crawler");
        self.crawler = self.crawler.start().await?;

        let result = match &self.task_type {
            TaskType::Auto(url) => {
                log::debug!("Executing auto crawl task for URL: {url}");
                self.crawl_auto(url).await
            }
            TaskType::Manual(codes) => {
                log::debug!("Executing manual crawl task for {} codes", codes.len());
                self.crawl_manual(codes).await
            }
        };

        match &result {
            Ok(_) => log::info!("{:?} Task execution completed successfully", self.task_type),
            Err(e) => log::error!("Task execution failed: {e}"),
        }

        result
    }

    async fn crawl_manual(&self, codes: &[String]) -> Result<(), AppError> {
        specified::crawl_codes(self.db.as_ref(), &self.crawler, codes).await
    }

    async fn crawl_auto(&self, url: &str) -> Result<(), AppError> {
        auto::auto_crawl_page(self.db.as_ref(), &self.crawler, url).await
    }
}
