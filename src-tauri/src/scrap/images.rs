use std::path::PathBuf;

use luneth::{crawl::WebCrawler, record::Recorder};
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

pub async fn crawl_record_image(
    app_handle: &AppHandle,
    crawler: &WebCrawler,
    recorder: &Recorder,
) -> Result<PathBuf, AppError> {
    let record_image_path = get_record_image_path(app_handle)?;
    let images = crawler.crawl_imgs_by_record(recorder).await?;
    let mut error_message = Vec::new();
    let mut save_path = None;
    for image in images.as_ref() {
        match image.save_to_dir_path(&record_image_path, Some(recorder.record.id.clone())) {
            Err(e) => {
                error_message.push(format!("{}: {}", image.name, e));
            }
            Ok(p) => {
                save_path = Some(p);
            }
        }
    }
    let error_message = error_message.join("\n");
    if !error_message.is_empty() {
        log::error!("Errors occurred while saving images:\n{error_message}");
        return Err(AppError::CrawlImageError(error_message));
    }
    let save_path = save_path.expect("No Image");
    Ok(save_path)
}
