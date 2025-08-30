use std::sync::Arc;

use luneth_db::DbOperator;
use serde::Serialize;
use tauri::{AppHandle, Emitter as _};

use crate::{common::new_postman, db::write::save_remote_records, handlers::TaskType, AppError};

impl super::Task {
    pub async fn new_pull_record_slim(app_handle: AppHandle, db: Arc<DbOperator>) -> Self {
        log::debug!("Creating new pull record slim task");
        let task_type = TaskType::PullRemote;
        log::debug!("Pull record slim task created successfully");
        Self {
            db,
            app_handle,
            task_type,
        }
    }

    pub(super) async fn pull_record_slim(&self) -> Result<(), AppError> {
        pull_record_slim(&self.app_handle, self.db.as_ref()).await
    }
}

async fn pull_record_slim(app_handle: &AppHandle, db: &DbOperator) -> Result<(), AppError> {
    let mut client = new_postman().await.map_err(|e| {
        let error_msg = format!("Failed to create client: {e}");
        report_record_pull_failed(app_handle, error_msg.clone());
        AppError::SendRequestFailed(error_msg)
    })?;

    log::info!("Starting record pull from remote server");

    let records = client.pull_remote_records_slim().await.map_err(|e| {
        let error_msg = format!("Failed to pull records from remote: {e}");
        report_record_pull_failed(app_handle, error_msg.clone());
        AppError::SendRequestFailed(error_msg)
    })?;

    let total_count = records.len();
    log::info!("Retrieved {total_count} records from remote server");

    // Report start event
    report_record_pull_start(app_handle, total_count);

    // Report progress
    report_record_pull_progress(
        app_handle,
        0,
        "Starting to save records to local database...".to_owned(),
    );

    let success_cnt = save_remote_records(db, records).await.map_err(|e| {
        let error_msg = format!("Failed to save records to database: {e}");
        report_record_pull_failed(app_handle, error_msg.clone());
        e
    })?;

    log::info!(
        "Successfully pulled {total_count} records from remote server, and saved {success_cnt}"
    );

    // Report completion
    report_record_pull_complete(app_handle, success_cnt, total_count);

    Ok(())
}

// ################
// # Report Events
// ###############

// Record pull events
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RecordPullStartEvent {
    total_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RecordPullProgressEvent {
    processed: usize,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RecordPullCompleteEvent {
    success_count: usize,
    total_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RecordPullFailedEvent {
    error_message: String,
}

// Event emission helper functions
fn report_record_pull_start(app_handle: &AppHandle, total_count: usize) {
    let event = RecordPullStartEvent { total_count };
    match app_handle.emit("record-pull-start", &event) {
        Ok(_) => log::debug!("Emitted record-pull-start event with {total_count} records"),
        Err(e) => log::error!("Failed to emit record-pull-start event: {e}"),
    }
}

fn report_record_pull_progress(app_handle: &AppHandle, processed: usize, message: String) {
    let event = RecordPullProgressEvent { processed, message };
    match app_handle.emit("record-pull-progress", &event) {
        Ok(_) => log::debug!("Emitted record-pull-progress event: {processed} processed"),
        Err(e) => log::error!("Failed to emit record-pull-progress event: {e}"),
    }
}

fn report_record_pull_complete(app_handle: &AppHandle, success_count: usize, total_count: usize) {
    let event = RecordPullCompleteEvent {
        success_count,
        total_count,
    };
    match app_handle.emit("record-pull-complete", &event) {
        Ok(_) => log::debug!(
            "Emitted record-pull-complete event: {success_count}/{total_count} successful"
        ),
        Err(e) => log::error!("Failed to emit record-pull-complete event: {e}"),
    }
}

fn report_record_pull_failed(app_handle: &AppHandle, error_message: String) {
    let event = RecordPullFailedEvent { error_message };
    match app_handle.emit("record-pull-failed", &event) {
        Ok(_) => log::debug!("Emitted record-pull-failed event"),
        Err(e) => log::error!("Failed to emit record-pull-failed event: {e}"),
    }
}
