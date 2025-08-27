use std::{sync::Arc, time::Duration};

use luneth::crawl::CrawlConfig;
use luneth_db::DbOperator;
use tauri::AppHandle;

use crate::{common::get_task_base_url, AppError};

mod auto;
mod batch;
mod idol;
mod images;
mod record;
mod submit;
mod update;

#[derive(Debug)]
pub struct AutoCrawlConfig {
    pub start_url: String,
    pub with_image: bool,
    pub crawl_config: CrawlConfig,
}

impl AutoCrawlConfig {
    pub async fn new(
        start_url: String,
        with_image: bool,
        headless: bool,
        load_timeout: u64,
        request_delay: u64,
        webdriver_port: u16,
    ) -> Result<Self, AppError> {
        let base_url = get_task_base_url().await?;
        Ok(Self {
            start_url,
            with_image,
            crawl_config: CrawlConfig {
                base_url,
                page_load_timeout: load_timeout,
                magnet_timeout: load_timeout,
                headless,
                request_delay: Duration::from_secs(request_delay),
                webdriver_port,
            },
        })
    }
}

#[derive(Debug)]
pub struct BatchCrawlConfig {
    pub batch: Vec<String>,
    pub with_image: bool,
    pub crawl_config: CrawlConfig,
}

impl BatchCrawlConfig {
    pub async fn new(
        batch: Vec<String>,
        with_image: bool,
        headless: bool,
        load_timeout: u64,
        request_delay: u64,
        webdriver_port: u16,
    ) -> Result<Self, AppError> {
        let base_url = get_task_base_url().await?;
        Ok(Self {
            batch,
            with_image,
            crawl_config: CrawlConfig {
                base_url,
                page_load_timeout: load_timeout,
                magnet_timeout: load_timeout,
                headless,
                request_delay: Duration::from_secs(request_delay),
                webdriver_port,
            },
        })
    }
}

#[derive(Debug)]
pub enum TaskType {
    // Start URL, and with image
    Auto(AutoCrawlConfig),

    // List of specified codes, and with image
    Batch(BatchCrawlConfig),

    // Pull remote records
    PullRemote,

    // Idol Link
    Idol,

    Submit(Vec<String>),

    Update(BatchCrawlConfig),
}

pub struct Task {
    app_handle: AppHandle,
    db: Arc<DbOperator>,
    task_type: TaskType,
}

impl Task {
    pub async fn exec(self) -> Result<(), AppError> {
        log::debug!("Starting task execution");
        log::debug!("Starting web crawler");

        let result = match &self.task_type {
            TaskType::Auto(config) => self.crawl_auto(config).await,
            TaskType::Batch(config) => self.crawl_batch(config).await,
            TaskType::Idol => self.crawl_idol().await,
            TaskType::PullRemote => self.pull_record_slim().await,
            TaskType::Submit(codes) => self.submit_codes(codes).await,
            TaskType::Update(config) => self.update_codes(config).await,
        };

        match &result {
            Ok(_) => log::info!("{:?} Task execution completed successfully", self.task_type),
            Err(e) => log::error!("Task execution failed: {e}"),
        }

        result
    }
}
