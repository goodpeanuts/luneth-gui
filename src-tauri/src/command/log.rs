use tauri::Manager as _;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_log_dir(app: tauri::AppHandle) -> Result<String, String> {
    let log_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("logs");

    Ok(log_dir.to_string_lossy().to_string())
}
