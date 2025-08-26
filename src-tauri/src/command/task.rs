#![expect(clippy::let_underscore_must_use)]

use crate::handlers::Task;
use crate::AppState;
use std::sync::Arc;
use tauri::State;

// ############
// # scraping
// #############

#[tauri::command(rename_all = "snake_case")]
pub async fn launch_auto_scrap_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    url: String,
    with_image: bool,
) -> Result<(), String> {
    log::debug!("Launching auto scraping task for URL: {url}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_auto(app, db, url.clone(), with_image).await?;
            log::debug!("Auto scraping task created for URL: {url}");
            task.exec().await?;
            log::debug!("Auto scraping task completed successfully for URL: {url}");
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
pub async fn launch_manual_scrap_task(
    app: tauri::AppHandle,
    state: State<'_, Arc<AppState>>,
    codes: Vec<String>,
    with_image: bool,
) -> Result<(), String> {
    log::debug!("Launching manual scraping task for {} codes", codes.len());
    log::debug!("Codes to scrape: {codes:?}");
    let db = Arc::clone(&state.db);

    // Use a blocking thread to handle non-Send types
    let handle = std::thread::spawn(move || {
        // Create a simple runtime for the async task
        let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
        rt.block_on(async move {
            let task = Task::new_manual(app, db, codes.clone(), with_image).await?;
            log::debug!("Manual scraping task created for {} codes", codes.len());
            task.exec().await?;
            log::debug!(
                "CommandManual scraping task completed successfully for {} codes",
                codes.len()
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
            let task = Task::new_idol(app, db).await?;
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
            let task = Task::new_pull_record_slim(app, db).await?;
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
            let task = Task::new_submit(app, db, codes.clone()).await?;
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
