#![expect(clippy::let_underscore_must_use)]

use std::sync::Arc;

use luneth_db::entities::record_local::Model as RecorderModel;
use tauri::{Manager as _, State};

use crate::{
    db::read::{get_op_history, get_records},
    AppState,
};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_records(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<RecorderModel>, String> {
    log::debug!("Fetching all records from database");
    let db = Arc::clone(&state.db);
    let records = get_records(db.as_ref()).await.map_err(|e| e.clone())?;
    log::info!("Retrieved {} records from database", records.len());
    Ok(records)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_op_history(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<luneth_db::history_op::Model>, String> {
    log::debug!("Fetching operation history from database");
    let db = Arc::clone(&state.db);
    let history = get_op_history(db.as_ref()).await.map_err(|e| e.clone())?;
    log::info!("Retrieved {} operation history records", history.len());
    Ok(history)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn mark_record_viewed(app: tauri::AppHandle, code: &str) -> Result<(), String> {
    let app_state = app.state::<std::sync::Arc<crate::AppState>>();
    let db = &app_state.db;

    crate::db::write::mark_record_viewed(db.as_ref(), code)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn mark_record_liked(app: tauri::AppHandle, code: &str) -> Result<(), String> {
    let app_state = app.state::<std::sync::Arc<crate::AppState>>();
    let db = &app_state.db;

    crate::db::write::mark_record_liked(db.as_ref(), code)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn mark_record_unliked(app: tauri::AppHandle, code: &str) -> Result<(), String> {
    let app_state = app.state::<std::sync::Arc<crate::AppState>>();
    let db = &app_state.db;

    crate::db::write::mark_record_unliked(db.as_ref(), code)
        .await
        .map_err(|e| e.to_string())
}
