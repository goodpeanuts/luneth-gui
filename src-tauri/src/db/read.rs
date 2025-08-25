use luneth_db::entities::record_local::Model as RecorderModel;
use luneth_db::{history_op, DbService};

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
