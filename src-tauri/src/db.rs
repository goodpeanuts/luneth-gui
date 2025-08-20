use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::{history_op, DbService, OperationStatus, OperationType};

use crate::AppError;

pub(crate) async fn get_records(db: &impl DbService) -> Result<Vec<RecorderModel>, String> {
    let records = db
        .query_entity::<luneth_db::record_local::Entity>(None, None)
        .await
        .map_err(|e| e.to_string())?;
    Ok(records)
}

pub(crate) async fn get_op_history(db: &impl DbService) -> Result<Vec<history_op::Model>, String> {
    let history = db
        .query_entity::<history_op::Entity>(None, None)
        .await
        .map_err(|e| e.to_string())?;
    Ok(history)
}

pub(crate) async fn log_success_crawl_op(db: &impl DbService, code: &str) -> Result<(), AppError> {
    let op_history_entry = history_op::Model::new_record(
        code.to_owned(),          // String
        OperationType::Create,    // OperationType
        OperationStatus::Success, // OperationStatus
        "crawl".to_owned(),       // String
        None,                     // Option<String> 错误信息
    );

    db.insert_entity(op_history_entry).await?;
    Ok(())
}

pub(crate) async fn log_failed_crawl_op(
    db: &impl DbService,
    code: &str,
    err: String,
) -> Result<(), AppError> {
    let op_history_entry = history_op::Model::new_record(
        code.to_owned(),         // String
        OperationType::Create,   // OperationType
        OperationStatus::Failed, // OperationStatus
        "crawl".to_owned(),      // String
        Some(err),               // Option<String> 错误信息
    );

    db.insert_entity(op_history_entry).await?;
    Ok(())
}
