#![expect(clippy::let_underscore_must_use)]

use std::sync::Arc;
use tauri::State;
use url::Url;

use crate::common::{ClientAuth, CLIENT_AUTH};
use crate::handlers::Task;
use crate::AppState;

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

    let client_auth = ClientAuth { url, id, secret };
    *CLIENT_AUTH.lock().await = Some(client_auth);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn clear_client_auth() -> Result<(), String> {
    *CLIENT_AUTH.lock().await = None;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn pull_record_slim(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_pull_record_slim(app, db).await?;
            log::debug!("Pull record slim task created ");
            task.exec().await?;
            log::debug!("Pull record slim task completed successfully",);
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
