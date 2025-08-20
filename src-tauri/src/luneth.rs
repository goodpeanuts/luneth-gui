#![expect(clippy::let_underscore_must_use)]

use std::sync::Arc;

use luneth_db::entities::record_local::Model as RecorderModel;
use tauri::State;
use url::Url;

use crate::db::{get_op_history, get_records};
use crate::scrap::{Task, TASK_BASE_URL};
use crate::AppState;

#[tauri::command(rename_all = "snake_case")]
pub async fn set_task_base_url(mut url: String) -> Result<(), String> {
    // Ensure the URL ends with /
    if url.is_empty() {
        log::info!("Task base URL reset");
    } else if !url.ends_with('/') {
        url.push('/');
    }
    Url::parse(&url).map_err(|e| format!("Invalid URL: {e}"))?;

    log::info!("Setting task base URL to: {url}");
    *TASK_BASE_URL.lock().await = Some(url);
    Ok(())
}

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
pub async fn launch_auto_scrap_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    url: String,
) -> Result<(), String> {
    log::debug!("Launching auto scraping task for URL: {url}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_auto(app, db, url.clone()).await?;
            log::debug!("Auto scraping task created for URL: {url}");
            task.exec().await?;
            log::debug!("Auto scraping task completed successfully for URL: {url}");
            Ok::<(), String>(())
        })
    });

    match handle.join().map_err(|_e| "Thread panicked".to_owned())? {
        Ok(_) => {
            log::info!("Command Auto scraping task thread completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Auto scraping task failed: {e}");
            Err(e)
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_manual_scrap_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    codes: Vec<String>,
) -> Result<(), String> {
    log::debug!("Launching manual scraping task for {} codes", codes.len());
    log::debug!("Codes to scrape: {codes:?}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_manual(app, db, codes.clone()).await?;
            log::debug!("Manual scraping task created for {} codes", codes.len());
            task.exec().await?;
            log::debug!(
                "CommandManual scraping task completed successfully for {} codes",
                codes.len()
            );
            Ok::<(), String>(())
        })
    });

    match handle.join().map_err(|_e| "Thread panicked".to_owned())? {
        Ok(_) => {
            log::info!("Manual scraping task thread completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Manual scraping task failed: {e}");
            Err(e)
        }
    }
}
