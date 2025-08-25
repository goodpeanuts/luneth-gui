use std::path::PathBuf;

use tauri::{AppHandle, Manager as _};

use crate::AppError;

pub fn get_local_image_path(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    // 获取应用本地数据目录 - Tauri v2 API
    match app_handle.path().app_local_data_dir() {
        Ok(p) => Ok(p.join("images")),
        Err(e) => Err(AppError::FileSystemError(e.to_string())),
    }
}

pub fn get_record_image_path(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    get_local_image_path(app_handle).map(|p| p.join("records"))
}

#[expect(unused)]
pub fn get_idol_image_path(app_handle: &AppHandle) -> Result<PathBuf, AppError> {
    get_local_image_path(app_handle).map(|p| p.join("idols"))
}
