use std::sync::Arc;

use luneth_db::DbOperator;
use tauri::AppHandle;

use crate::AppError;

mod auto;
mod batch;
mod idol;
mod images;
mod record;
mod submit;
mod update;

#[derive(Debug)]
pub enum TaskType {
    // Start URL, and with image
    Auto(String, bool),

    // List of specified codes, and with image
    Batch(Vec<String>, bool),

    // Pull remote records
    PullRemote,

    // Idol Link
    Idol,

    Submit(Vec<String>),

    Update(Vec<String>),
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
            TaskType::Auto(url, with_image) => {
                log::debug!("Executing auto crawl task for URL: {url}");
                self.crawl_auto(url, *with_image).await
            }
            TaskType::Batch(codes, with_image) => {
                log::debug!("Executing manual crawl task for {} codes", codes.len());
                self.crawl_batch(codes, *with_image).await
            }
            TaskType::Idol => {
                log::debug!("Executing idol crawl task");
                self.crawl_idol().await
            }
            TaskType::PullRemote => self.pull_record_slim().await,
            TaskType::Submit(codes) => {
                log::debug!("Executing submit crawl task for {} codes", codes.len());
                self.submit_codes(codes).await
            }
            TaskType::Update(codes) => {
                log::debug!("Executing update crawl task for {} codes", codes.len());
                self.update_codes(codes).await
            }
        };

        match &result {
            Ok(_) => log::info!("{:?} Task execution completed successfully", self.task_type),
            Err(e) => log::error!("Task execution failed: {e}"),
        }

        result
    }
}
