use std::path::PathBuf;

use luneth::{common::ImageData, crawl::WebCrawler, record::Recorder};
use tauri::AppHandle;

use crate::{common::get_record_image_path, AppError};

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

pub async fn read_local_images(
    app_handle: &AppHandle,
    id: &str,
    cnt: i32,
) -> Result<Vec<ImageData>, AppError> {
    let image_base_path = get_record_image_path(app_handle)?;
    let mut images = Vec::new();

    let display_image = ImageData::read_from_dir_path(&image_base_path, Some(id.to_owned()), id)?;
    images.push(display_image);

    for i in 0..cnt - 1 {
        let image = ImageData::read_from_dir_path(
            &image_base_path,
            Some(id.to_owned()),
            &format!("{id}_{i}"),
        )?;
        images.push(image);
    }

    Ok(images)
}
