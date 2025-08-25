use luneth_db::{history_op, DbService, OperationStatus, OperationType};

use crate::AppError;

pub(crate) async fn log_success_op(
    db: &impl DbService,
    op_type: OperationType,
    code: &str,
) -> Result<(), AppError> {
    log::debug!("Logging successful operation for code: {code} and type: {op_type}");
    let op_history_entry = history_op::Model::new_record(
        code.to_owned(),          // String
        op_type,                  // OperationType
        OperationStatus::Success, // OperationStatus
        "crawl".to_owned(),       // String
        None,                     // Option<String> 错误信息
    );

    db.insert_entity(op_history_entry).await?;
    Ok(())
}

pub(crate) async fn log_failed_op(
    db: &impl DbService,
    op_type: OperationType,
    code: &str,
    err: String,
) -> Result<(), AppError> {
    log::error!("Logging failed {op_type} operation for  {code},  with error: {err}");
    let op_history_entry = history_op::Model::new_record(
        code.to_owned(),         // String
        op_type,                 // OperationType
        OperationStatus::Failed, // OperationStatus
        "crawl".to_owned(),      // String
        Some(err),               // Option<String> 错误信息
    );

    db.insert_entity(op_history_entry).await?;
    Ok(())
}
