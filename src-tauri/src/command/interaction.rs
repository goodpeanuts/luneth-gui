#![expect(clippy::let_underscore_must_use)]

use tauri::Manager as _;

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
