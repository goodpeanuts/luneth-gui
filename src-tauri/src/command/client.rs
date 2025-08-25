#![expect(clippy::let_underscore_must_use)]

use std::sync::Arc;
use tauri::State;
use url::Url;

use crate::client::CLIENT_AUTH;
use crate::db::save_remote_records;
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
