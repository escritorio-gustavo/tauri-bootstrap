// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]
#![deny(clippy::enum_glob_use)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]

use browser_manager::BrowserManagerState;
use lazy_static::lazy_static;
use std::{path::{Path, PathBuf}, sync::Arc};
use tauri::{async_runtime::block_on, Manager, State, WindowEvent};
use tokio::sync::{OnceCell, RwLock};

pub mod browser_manager;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;
pub mod scraper;

pub static EXECUTABLE_PATH: OnceCell<PathBuf> = OnceCell::const_new();

lazy_static! {
    pub static ref DATA_SET: RwLock<Vec<Arc<()>>> = RwLock::new(vec![]);
}

#[tokio::main]
async fn main() {
    let browser_manager_state = BrowserManagerState::new();

    tauri::Builder::default()
        .setup(|app| {
            block_on(async move {
                let dir = app
                    .handle()
                    .path_resolver()
                    .app_local_data_dir()
                    .ok_or(prelude::Error::AppLocalDataDir)?;
                let browser_path = dir.join("browser");

                download_browser(&browser_path).await
            }).expect("Failed to setup browser");
            Ok(())
        })
        .manage(browser_manager_state)
        .on_window_event(move |event| {
            if matches!(event.event(), WindowEvent::Destroyed) {
                block_on(async {
                    let state: State<BrowserManagerState> = event.window().state();
                    state.browser_manager_mutex.lock().await.clear();
                });
            }
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn download_browser(browser_path: &Path) -> prelude::Result<()> {
    let _ = std::fs::create_dir_all(browser_path);
    let fetcher = chromiumoxide::BrowserFetcherOptions::builder()
        .with_path(browser_path)
        .build()
        .map(chromiumoxide::BrowserFetcher::new)?;

    let info = fetcher.fetch().await?;

    EXECUTABLE_PATH
        .set(info.executable_path)
        .map_err(Into::into)
}
