use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::impl_local::LocalFilterCondition;
use luneth_db::{history_op, DbOperator};

use crate::AppError;

pub(crate) async fn get_records_count(
    db: &DbOperator,
    filters: Vec<String>,
) -> Result<u64, AppError> {
    let filters = filters
        .into_iter()
        .filter_map(|f| f.parse().ok())
        .collect::<Vec<LocalFilterCondition>>();
    log::debug!("Querying records count from database, filter by {filters:?}");
    let count = db.query_total_count(filters).await?;
    log::debug!("Successfully retrieved records count {count}");

    Ok(count)
}

pub(crate) async fn get_local_records(
    db: &DbOperator,
    offset: Option<u64>,
    limit: Option<u64>,
    filters: Vec<String>,
) -> Result<Vec<RecorderModel>, AppError> {
    let filters = filters
        .into_iter()
        .filter_map(|f| f.parse().ok())
        .collect::<Vec<LocalFilterCondition>>();
    log::debug!("Querying records count from database, filter by {filters:?}");
    let records = db.query_local(offset, limit, filters).await?;
    log::debug!("Successfully retrieved {} records", records.len());
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
