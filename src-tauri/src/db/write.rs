use luneth_db::DbService;
use luneth_db::OperationType;

use crate::AppError;

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
                if e.to_string().contains("UNIQUE constraint") {
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

pub(crate) async fn mark_record_viewed(db: &impl DbService, code: &str) -> Result<(), AppError> {
    log::debug!("Marking record as viewed: {code}");
    use luneth_db::entities::record_local::Entity as RecordLocal;

    let record_local = db.find_record_local_by_id::<RecordLocal>(code).await?;

    if let Some(record) = record_local {
        let viewed_am = record.set_viewd(true);
        db.update_record_local(viewed_am).await?;
        super::log::log_success_op(db, OperationType::Viewed, code).await?;
    } else {
        log::error!("Record with code {code} not found in local database");
        super::log::log_failed_op(
            db,
            OperationType::Viewed,
            code,
            "Record not found".to_owned(),
        )
        .await?;
    }

    Ok(())
}

pub(crate) async fn mark_record_liked(db: &impl DbService, code: &str) -> Result<(), AppError> {
    log::debug!("Marking record as liked: {code}");
    use luneth_db::entities::record_local::Entity as RecordLocal;

    let record_local = db.find_record_local_by_id::<RecordLocal>(code).await?;

    if let Some(record) = record_local {
        let liked_am = record.set_liked(true);
        db.update_record_local(liked_am).await?;
        super::log::log_success_op(db, OperationType::Liked, code).await?;
    } else {
        log::error!("Record with code {code} not found in local database");
        super::log::log_failed_op(
            db,
            OperationType::Liked,
            code,
            "Record not found".to_owned(),
        )
        .await?;
    }

    Ok(())
}

pub(crate) async fn mark_record_unliked(db: &impl DbService, code: &str) -> Result<(), AppError> {
    log::debug!("Marking record as unliked: {code}");
    use luneth_db::entities::record_local::Entity as RecordLocal;

    let record_local = db.find_record_local_by_id::<RecordLocal>(code).await?;

    if let Some(record) = record_local {
        let unliked_am = record.set_liked(false);
        db.update_record_local(unliked_am).await?;
        super::log::log_success_op(db, OperationType::Unliked, code).await?;
    } else {
        log::error!("Record with code {code} not found in local database");
        super::log::log_failed_op(
            db,
            OperationType::Unliked,
            code,
            "Record not found".to_owned(),
        )
        .await?;
    }

    Ok(())
}
