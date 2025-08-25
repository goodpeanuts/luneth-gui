use luneth::common::ImageData;
use tauri::Manager as _;

use crate::common::get_record_image_path;

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
pub async fn read_local_record_image(
    app: tauri::AppHandle,
    record_id: String,
    image_index: Option<i32>,
) -> Result<Option<Vec<u8>>, String> {
    let image_base_path =
        get_record_image_path(&app).map_err(|e| format!("Failed to get image base path: {e}"))?;

    let file_name = if let Some(index) = image_index {
        format!("{record_id}_{index}")
    } else {
        record_id.clone()
    };

    match ImageData::read_from_dir_path(&image_base_path, Some(record_id), &file_name) {
        Ok(image_data) => Ok(Some(image_data.bytes)),
        Err(e) => {
            log::debug!("Failed to read local image {file_name}: {e}");
            Ok(None)
        }
    }
}
