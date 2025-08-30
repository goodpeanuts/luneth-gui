use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::{history_op, DbOperator};

use crate::AppError;

#[expect(unused)]
pub(crate) async fn get_records(db: &DbOperator) -> Result<Vec<RecorderModel>, AppError> {
    log::debug!("Querying all records from database");
    let records = db.query_local(None, None).await?;
    log::debug!("Successfully retrieved {} records", records.len());
    Ok(records)
}

pub(crate) async fn get_records_ordered_by_updated_at(
    db: &DbOperator,
) -> Result<Vec<RecorderModel>, AppError> {
    log::debug!("Querying all records from database ordered by updated_at desc");

    let records = db.get_records_ordered_by_updated_at().await?;
    Ok(records)
}

// TODO: Op errortype display
pub(crate) async fn get_op_history(db: &DbOperator) -> Result<Vec<history_op::Model>, AppError> {
    log::debug!("Querying operation history from database");
    let history = db.query_history_op().await?;
    log::debug!(
        "Successfully retrieved {} operation history entries",
        history.len()
    );
    Ok(history)
}

pub(crate) async fn get_remote_record_id(db: &DbOperator) -> Result<Vec<String>, AppError> {
    log::debug!("Querying all remote record IDs from database");
    let ids = db
        .query_specified_column(luneth_db::record_remote::Column::Id)
        .await?;
    log::debug!("Successfully retrieved {} remote record IDs", ids.len());
    Ok(ids)
}

pub(crate) async fn get_local_record_id(db: &DbOperator) -> Result<Vec<String>, AppError> {
    log::debug!("Querying all local record IDs from database");
    let ids = db
        .query_specified_column(luneth_db::record_local::Column::Id)
        .await?;
    log::debug!("Successfully retrieved {} local record IDs", ids.len());
    Ok(ids)
}

pub(crate) async fn get_exist_record_ids(db: &DbOperator) -> Result<Vec<String>, AppError> {
    log::debug!("Querying all existing record IDs from database");
    let remote_ids = get_remote_record_id(db).await?;
    let local_ids = get_local_record_id(db).await?;
    let mut exist_ids: Vec<String> = remote_ids;
    exist_ids.extend(local_ids);
    log::debug!(
        "Successfully retrieved {} existing record IDs",
        exist_ids.len()
    );
    Ok(exist_ids)
}
