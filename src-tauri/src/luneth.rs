#![expect(clippy::let_underscore_must_use)]

use std::sync::Arc;

use luneth_db::entities::record_local::Model as RecorderModel;
use tauri::{Manager as _, State};
use url::Url;

use crate::client::CLIENT_AUTH;
use crate::db::{get_op_history, get_records, save_remote_records};
use crate::scrap::{Task, TASK_BASE_URL};
use crate::AppState;

// ############
// # scraping
// #############

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
    with_image: bool,
) -> Result<(), String> {
    log::debug!("Launching auto scraping task for URL: {url}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_auto(app, db, url.clone(), with_image).await?;
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
    with_image: bool,
) -> Result<(), String> {
    log::debug!("Launching manual scraping task for {} codes", codes.len());
    log::debug!("Codes to scrape: {codes:?}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_manual(app, db, codes.clone(), with_image).await?;
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

// ############
// # client
// #############

#[tauri::command(rename_all = "snake_case")]
pub async fn set_client_auth(mut url: String, id: String, secret: String) -> Result<(), String> {
    // Ensure the URL ends with /
    if url.is_empty() {
        log::info!("Task base URL reset");
    } else if !url.ends_with('/') {
        url.push('/');
    }
    Url::parse(&url).map_err(|e| format!("Invalid URL: {e}"))?;

    let client_auth = crate::client::ClientAuth { url, id, secret };
    *CLIENT_AUTH.lock().await = Some(client_auth);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn clear_client_auth() -> Result<(), String> {
    *CLIENT_AUTH.lock().await = None;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn pull_record_slim(state: State<'_, Arc<AppState>>) -> Result<(usize, usize), String> {
    log::debug!("Pulling record slim data from remote server");
    let records = crate::client::pull_record_slim()
        .await
        .map_err(|e| e.to_string())?;

    let len = records.len();
    let db = Arc::clone(&state.db);

    let success_cnt = save_remote_records(db.as_ref(), records)
        .await
        .map_err(|e| e.to_string())?;
    log::info!("Successfully pulled {len} records from remote server",);
    Ok((len, success_cnt))
}

// ############
// # local images
// ############

#[tauri::command(rename_all = "snake_case")]
pub async fn get_app_local_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    let local_data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data dir: {e}"))?;

    Ok(local_data_dir.to_string_lossy().to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn check_local_image_exists(
    app: tauri::AppHandle,
    record_id: String,
    image_index: i32,
) -> Result<Option<String>, String> {
    let local_data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data dir: {e}"))?;

    let images_dir = local_data_dir.join("images").join(&record_id);

    // Check for different file extensions
    let extensions = ["jpg", "jpeg", "png", "gif", "webp", "bmp"];
    let filename_base = if image_index == -1 {
        // Display image (cover thumbnail)
        record_id.clone()
    } else {
        // Cover (index 0) or sample images (index 1, 2, ...)
        format!("{record_id}_{image_index}")
    };

    for ext in &extensions {
        let file_path = images_dir.join(format!("{filename_base}.{ext}"));
        if file_path.exists() {
            return Ok(Some(file_path.to_string_lossy().to_string()));
        }
    }

    Ok(None)
}
