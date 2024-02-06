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

lazy_static! {
    pub static ref DATA_SET: RwLock<Vec<Arc<()>>> = RwLock::new(vec![]);
}

pub struct BrowserManagerState {
    browser_manager_mutex: Arc<Mutex<BrowserManager>>,
}

fn main() {
    let browser_manager_state = BrowserManagerState::new();


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
