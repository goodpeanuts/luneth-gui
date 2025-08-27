#![expect(clippy::let_underscore_must_use)]

use crate::handlers::Task;
use crate::AppState;
use std::sync::Arc;
use tauri::State;

// ############
// # scraping
// #############

#[tauri::command(rename_all = "snake_case")]
#[expect(clippy::too_many_arguments)]
pub async fn launch_auto_scrap_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    start_url: String,
    with_image: bool,
    headless: bool,
    load_timeout: u64,
    request_delay: u64,
    webdriver_port: u16,
) -> Result<(), String> {
    log::debug!("Launching auto scraping task for URL: {start_url}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_auto(
                app,
                db,
                start_url.clone(),
                with_image,
                headless,
                load_timeout,
                request_delay,
                webdriver_port,
            )
            .await
            .map_err(|e| e.to_string())?;
            log::debug!("Auto scraping task created for URL: {start_url}");
            task.exec().await.map_err(|e| e.to_string())?;
            log::debug!("Auto scraping task completed successfully for URL: {start_url}");
            Ok::<(), String>(())
        })
    });

    match handle.join().map_err(|_e| "Thread panicked".to_owned())? {
        Ok(_) => {
            log::info!("Command Auto scraping task thread completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Auto scraping task failed: {e}");
            Err(e)
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
#[expect(clippy::too_many_arguments)]
pub async fn launch_batch_scrap_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    batch: Vec<String>,
    with_image: bool,
    headless: bool,
    load_timeout: u64,
    request_delay: u64,
    webdriver_port: u16,
) -> Result<(), String> {
    log::debug!("Launching manual scraping task for {} codes", batch.len());
    log::debug!("Codes to scrape: {batch:?}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_manual(
                app,
                db,
                batch.clone(),
                with_image,
                headless,
                load_timeout,
                request_delay,
                webdriver_port,
            )
            .await
            .map_err(|e| e.to_string())?;
            log::debug!("Manual scraping task created for {} codes", batch.len());
            task.exec().await.map_err(|e| e.to_string())?;
            log::debug!(
                "Command Manual scraping task completed successfully for {} codes",
                batch.len()
            );
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

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_idol_scrap_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = Arc::clone(&state.db);
    let handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_idol(app, db).await;
            log::debug!("Idol scraping task created");
            task.exec().await?;
            log::debug!("Idol scraping task completed successfully");
            Ok::<(), String>(())
        })
    });

    match handle.join().map_err(|_e| "Thread panicked".to_owned())? {
        Ok(_) => {
            log::info!("Idol scraping task thread completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Idol scraping task failed: {e}");
            Err(e)
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_record_pull_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let db = Arc::clone(&state.db);
    let handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_pull_record_slim(app, db).await;
            log::debug!("Record pull task created");
            task.exec().await?;
            log::debug!("Record pull task completed successfully");
            Ok::<(), String>(())
        })
    });

    match handle.join().map_err(|_e| "Thread panicked".to_owned())? {
        Ok(_) => {
            log::info!("Record pull task thread completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Record pull task failed: {e}");
            Err(e)
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_submit_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    codes: Vec<String>,
) -> Result<(), String> {
    log::debug!("Launching submit task for {} codes", codes.len());
    log::debug!("Codes to submit: {codes:?}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_submit(app, db, codes.clone()).await;
            log::debug!("Submit task created for {} codes", codes.len());
            task.exec().await?;
            log::debug!(
                "Submit task completed successfully for {} codes",
                codes.len()
            );
            Ok::<(), String>(())
        })
    });

    match handle.join().map_err(|_e| "Thread panicked".to_owned())? {
        Ok(_) => {
            log::info!("Submit task thread completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Submit task failed: {e}");
            Err(e)
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
#[expect(clippy::too_many_arguments)]
pub async fn launch_update_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    batch: Vec<String>,
    with_image: bool,
    headless: bool,
    load_timeout: u64,
    request_delay: u64,
    webdriver_port: u16,
) -> Result<(), String> {
    log::debug!("Launching update task for {} codes", batch.len());
    log::debug!("Codes to update: {batch:?}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let config = crate::handlers::BatchCrawlConfig::new(
                batch.clone(),
                with_image,
                headless,
                load_timeout,
                request_delay,
                webdriver_port,
            )
            .await
            .map_err(|e| e.to_string())?;

            let task = Task::new_update(app, db, config)
                .await
                .map_err(|e| e.to_string())?;
            log::debug!("Update task created for {} codes", batch.len());
            task.exec().await.map_err(|e| e.to_string())?;
            log::debug!(
                "Update task completed successfully for {} codes",
                batch.len()
            );
            Ok::<(), String>(())
        })
    });

    match handle.join().map_err(|_e| "Thread panicked".to_owned())? {
        Ok(_) => {
            log::info!("Update task thread completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Update task failed: {e}");
            Err(e)
        }
    }
}
