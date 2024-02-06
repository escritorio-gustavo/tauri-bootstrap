// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use browser_manager::BrowserManagerState;
use lazy_static::lazy_static;
use std::sync::Arc;
use tauri::{Manager, State, WindowEvent};
use tokio::sync::{Mutex, RwLock};

pub mod browser_manager;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;
pub mod scraper;

pub static EXECUTABLE_PATH: OnceCell<PathBuf> = OnceCell::const_new();
lazy_static! {
    pub static ref TAURI_CONFIG: tauri::Config =
        serde_json::from_str(include_str!("../tauri.conf.json")).unwrap();
    pub static ref BROWSER_PATH: PathBuf = tauri::api::path::app_local_data_dir(&TAURI_CONFIG)
        .unwrap()
        .join("browser");
    pub static ref DATA_SET: RwLock<Vec<Arc<()>>> = RwLock::new(vec![]);
}

#[tokio::main]
fn main() {
    let browser_manager_state = BrowserManagerState::new();

    download_browser().await;

    tauri::Builder::default()
        .manage(browser_manager_state)
        .on_window_event(move |event| {
            if let WindowEvent::Destroyed = event.event() {
                tauri::async_runtime::block_on(async {
                    let state: State<BrowserManagerState> = event.window().state();
                    state.browser_manager_mutex.lock().await.clear();
                })
            }
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn download_browser() {
    let _ = std::fs::create_dir_all(BROWSER_PATH.as_path());
    let fetcher = chromiumoxide::BrowserFetcherOptions::builder()
        .with_path(BROWSER_PATH.as_path())
        .build()
        .map(chromiumoxide::BrowserFetcher::new)
        .expect("Path must be provided to build BrowserFetcherOptions");

    match fetcher.fetch().await {
        Ok(info) => {
            EXECUTABLE_PATH
                .set(info.executable_path)
                .expect("This should only happen once");
        }
        Err(e) => {
            eprintln!("Fetcher failed: {e}");
        }
    };
}
