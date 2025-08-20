use std::sync::Arc;

use luneth::crawl::{self, crawler};
use luneth_db::DbOperator;

use crate::AppError;

mod auto;
mod specified;

pub enum TaskType {
    // Start URL
    Auto(String),

    Manual(Vec<String>),
}

pub struct Task {
    db: Arc<DbOperator>,
    task_type: TaskType,
    crawler: crawler::WebCrawler,
}

impl Task {
    pub fn new_auto(db: Arc<DbOperator>, start_url: String) -> Result<Self, AppError> {
        let task_type = TaskType::Auto(start_url);
        let crawler = crawl::WebCrawler::new()?;
        Ok(Self {
            db,
            task_type,
            crawler,
        })
    }

    pub fn new_manual(db: Arc<DbOperator>, codes: Vec<String>) -> Result<Self, AppError> {
        let task_type = TaskType::Manual(codes);
        let crawler = crawl::WebCrawler::new()?;
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
        self.crawler = self.crawler.start().await?;
        match &self.task_type {
            TaskType::Auto(url) => self.crawl_auto(url).await,
            TaskType::Manual(codes) => self.crawl_manual(codes).await,
        }
    }

    async fn crawl_manual(&self, codes: &[String]) -> Result<(), AppError> {
        specified::crawl_codes(self.db.as_ref(), &self.crawler, codes).await
    }

    async fn crawl_auto(&self, url: &str) -> Result<(), AppError> {
        auto::auto_crawl_page(self.db.as_ref(), &self.crawler, url).await
    }
}
