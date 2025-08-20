#![expect(clippy::let_underscore_must_use)]

use std::sync::Arc;

use luneth_db::entities::record_local::Model as RecorderModel;
use tauri::State;

use crate::db::{get_op_history, get_records};
use crate::scrap::Task;
use crate::AppState;

#[tauri::command(rename_all = "snake_case")]
pub fn set_crawl_base_url(url: String) {
    // This function is a placeholder for setting the base URL for crawling.
    // The actual implementation would depend on how the luneth library allows setting the base URL.
    // For now, we just log the URL.
    log::debug!("Setting crawl base URL to: {url}");
    std::env::set_var("BASE_URL", url);
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_records(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<RecorderModel>, String> {
    let db = Arc::clone(&state.db);
    let records = get_records(db.as_ref()).await.map_err(|e| e.clone())?;
    Ok(records)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_op_history(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<luneth_db::history_op::Model>, String> {
    let db = Arc::clone(&state.db);
    let history = get_op_history(db.as_ref()).await.map_err(|e| e.clone())?;
    Ok(history)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_auto_scrap_task(
    state: State<'_, Arc<AppState>>,
    url: String,
) -> Result<(), String> {
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_auto(db, url)?;
            task.exec().await?;
            Ok::<(), String>(())
        })
    });

    handle.join().map_err(|_e| "Thread panicked".to_owned())?
}

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_manual_scrap_task(
    state: State<'_, Arc<AppState>>,
    codes: Vec<String>,
) -> Result<(), String> {
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_manual(db, codes)?;
            task.exec().await?;
            Ok::<(), String>(())
        })
    });

    handle.join().map_err(|_e| "Thread panicked".to_owned())?
}
