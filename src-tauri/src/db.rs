use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::{history_op, DbService, OperationStatus, OperationType};

use crate::AppError;

pub(crate) async fn get_records(db: &impl DbService) -> Result<Vec<RecorderModel>, String> {
    log::debug!("Querying all records from database");
    let records = db
        .query_entity::<luneth_db::record_local::Entity>(None, None)
        .await
        .map_err(|e| {
            log::error!("Failed to query records: {e}");
            e.to_string()
        })?;
    log::debug!("Successfully retrieved {} records", records.len());
    Ok(records)
}

// TODO: Op errortype display
pub(crate) async fn get_op_history(db: &impl DbService) -> Result<Vec<history_op::Model>, String> {
    log::debug!("Querying operation history from database");
    let history = db
        .query_entity::<history_op::Entity>(None, None)
        .await
        .map_err(|e| {
            log::error!("Failed to query operation history: {e}");
            e.to_string()
        })?;
    log::debug!(
        "Successfully retrieved {} operation history entries",
        history.len()
    );
    Ok(history)
}

pub(crate) async fn log_success_crawl_record_op(
    db: &impl DbService,
    code: &str,
) -> Result<(), AppError> {
    log::debug!("Logging successful crawl operation for code: {code}");
    let op_history_entry = history_op::Model::new_record(
        code.to_owned(),            // String
        OperationType::CrawlRecord, // OperationType
        OperationStatus::Success,   // OperationStatus
        "crawl".to_owned(),         // String
        None,                       // Option<String> 错误信息
    );

    db.insert_entity(op_history_entry).await?;
    Ok(())
}

pub(crate) async fn log_success_crawl_page_op(
    db: &impl DbService,
    page: &str,
) -> Result<(), AppError> {
    log::debug!("Logging successful crawl operation for page: {page}");
    let op_history_entry = history_op::Model::new_record(
        page.to_owned(),          // String
        OperationType::CrawlPage, // OperationType
        OperationStatus::Success, // OperationStatus
        "crawl".to_owned(),       // String
        None,                     // Option<String> 错误信息
    );

    db.insert_entity(op_history_entry).await?;
    Ok(())
}

pub(crate) async fn log_failed_crawl_record_op(
    db: &impl DbService,
    code: &str,
    err: String,
) -> Result<(), AppError> {
    log::debug!("Logging failed crawl operation for code: {code} with error: {err}");
    let op_history_entry = history_op::Model::new_record(
        code.to_owned(),            // String
        OperationType::CrawlRecord, // OperationType
        OperationStatus::Failed,    // OperationStatus
        "crawl".to_owned(),         // String
        Some(err),                  // Option<String> 错误信息
    );

    db.insert_entity(op_history_entry).await?;
    Ok(())
}

pub(crate) async fn log_failed_crawl_page_op(
    db: &impl DbService,
    page: &str,
    err: String,
) -> Result<(), AppError> {
    log::debug!("Logging failed crawl operation for code: {page} with error: {err}");
    let op_history_entry = history_op::Model::new_record(
        page.to_owned(),          // String
        OperationType::CrawlPage, // OperationType
        OperationStatus::Failed,  // OperationStatus
        "crawl".to_owned(),       // String
        Some(err),                // Option<String> 错误信息
    );

    db.insert_entity(op_history_entry).await?;
    Ok(())
}

// ############
// # client
// #############
pub(crate) async fn save_remote_records(
    db: &impl DbService,
    records: Vec<luneth::common::RecordSlimDto>,
) -> Result<usize, AppError> {
    log::debug!("Saving {} remote records to local database", records.len());
    let mut success_cnt: usize = 0;
    for record in records {
        let id = record.id.clone();
        let active_model = luneth_db::entities::record_remote::ActiveModel::from(record);
        match db.insert_entity(active_model).await {
            Ok(_) => success_cnt += 1,
            Err(e) => {
                if e.to_string().contains("duplicate key") {
                    log::debug!("Record already exists, skipping: {id}");
                } else {
                    log::error!("Failed to save record {id}: {e}");
                    return Err(e.into());
                }
            }
        }
    }
    log::info!("Successfully saved remote records to local database");
    Ok(success_cnt)
}
