use std::{path::PathBuf, sync::LazyLock};

use luneth::client::Postman;
use tauri::{AppHandle, Manager as _};
use tokio::sync::Mutex;

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

pub static CLIENT_AUTH: LazyLock<Mutex<Option<ClientAuth>>> = LazyLock::new(|| {
    Mutex::new(None) // Default base URL
});

#[derive(Clone, Debug)]
pub struct ClientAuth {
    pub url: String,
    pub id: String,
    pub secret: String,
}

pub async fn new_postman() -> Result<Postman, AppError> {
    let Some(auth) = CLIENT_AUTH.lock().await.clone() else {
        return Err(AppError::GetAuthFailed(
            "Client authentication is not set".into(),
        ));
    };
    let client = Postman::new(&auth.url, auth.id, auth.secret);
    Ok(client)
}
