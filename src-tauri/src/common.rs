use std::{path::PathBuf, sync::LazyLock};

use luneth::{
    client::Postman,
    crawl::{CrawlConfig, WebCrawler},
};
use luneth_db::DbService;
use tauri::{AppHandle, Manager as _};
use tokio::sync::{Mutex, RwLock};

use crate::{db::read::get_exist_record_ids, AppError};

pub static EXIST_IDS: LazyLock<RwLock<ExistIDs>> =
    LazyLock::new(|| RwLock::new(ExistIDs::default()));

#[derive(Default)]
pub struct ExistIDs {
    pub ids: Vec<String>,
}

impl ExistIDs {
    pub async fn fresh(&mut self, db: &impl DbService) {
        match get_exist_record_ids(db).await {
            Ok(records) => self.ids = records,
            Err(e) => log::error!("Failed to refresh existing record IDs: {e}"),
        };
    }
}

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

pub static TASK_BASE_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| {
    Mutex::new(None) // Default base URL
});

pub async fn get_task_base_url() -> Result<String, AppError> {
    let Some(base_url) = TASK_BASE_URL.lock().await.clone() else {
        return Err(AppError::GetAuthFailed("Task base URL is not set".into()));
    };
    Ok(base_url)
}

pub async fn new_crawler() -> Result<WebCrawler, AppError> {
    log::debug!("Creating new web crawler");
    let base_url = get_task_base_url().await?;

    log::debug!("Creating crawler with base URL: {base_url}");
    let config = luneth::crawl::CrawlConfig {
        base_url: base_url.clone(),
        ..Default::default()
    };
    WebCrawler::with_config(config).map_err(AppError::CrawlError)
}

pub async fn new_crawler_with_config(config: CrawlConfig) -> Result<WebCrawler, AppError> {
    log::debug!("Creating crawler with config: {config:?}");
    WebCrawler::with_config(config).map_err(AppError::CrawlError)
}
