use std::path::PathBuf;

use luneth::{crawl::WebCrawler, record::Recorder};
use tauri::{AppHandle, Manager as _};

use crate::AppError;

fn get_image_save_path(app_handle: &AppHandle, id: &str) -> Result<PathBuf, AppError> {
    // 获取应用本地数据目录 - Tauri v2 API
    match app_handle.path().app_local_data_dir() {
        Ok(p) => Ok(p.join("images").join(id)),
        Err(e) => Err(AppError::FileSystemError(e.to_string())),
    }
}

pub async fn crawl_image(
    app_handle: &AppHandle,
    crawler: &WebCrawler,
    recorder: &Recorder,
) -> Result<PathBuf, AppError> {
    let save_path = get_image_save_path(app_handle, &recorder.id)?;
    let images = crawler.crawl_imgs_by_record(recorder).await?;
    let mut error_message = Vec::new();
    for image in images.as_ref() {
        if let Err(e) = image.save_to_dir_path(&save_path) {
            error_message.push(format!("{}: {}", image.name, e));
        }
    }
    let error_message = error_message.join("\n");
    if !error_message.is_empty() {
        log::error!("Errors occurred while saving images:\n{error_message}");
        return Err(AppError::CrawlImageError(error_message));
    }

    Ok(save_path)
}
