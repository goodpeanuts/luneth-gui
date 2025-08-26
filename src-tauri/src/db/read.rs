use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::{history_op, DbService};

use crate::AppError;

pub(crate) async fn get_records(db: &impl DbService) -> Result<Vec<RecorderModel>, AppError> {
    log::debug!("Querying all records from database");
    let records = db
        .query_entity::<luneth_db::record_local::Entity>(None, None)
        .await?;
    log::debug!("Successfully retrieved {} records", records.len());
    Ok(records)
}

// TODO: Op errortype display
pub(crate) async fn get_op_history(
    db: &impl DbService,
) -> Result<Vec<history_op::Model>, AppError> {
    log::debug!("Querying operation history from database");
    let history = db.query_entity::<history_op::Entity>(None, None).await?;
    log::debug!(
        "Successfully retrieved {} operation history entries",
        history.len()
    );
    Ok(history)
}

pub(crate) async fn get_remote_record_id(db: &impl DbService) -> Result<Vec<String>, AppError> {
    log::debug!("Querying all remote record IDs from database");
    use luneth_db::entities::record_remote;
    let ids = db
        .query_specified_column::<record_remote::Entity, _, String>(record_remote::Column::Id)
        .await?;
    log::debug!("Successfully retrieved {} remote record IDs", ids.len());
    Ok(ids)
}

pub(crate) async fn get_local_record_id(db: &impl DbService) -> Result<Vec<String>, AppError> {
    log::debug!("Querying all local record IDs from database");
    use luneth_db::entities::record_local;
    let ids = db
        .query_specified_column::<record_local::Entity, _, String>(record_local::Column::Id)
        .await?;
    log::debug!("Successfully retrieved {} local record IDs", ids.len());
    Ok(ids)
}

pub(crate) async fn get_exist_record_ids(db: &impl DbService) -> Result<Vec<String>, AppError> {
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
