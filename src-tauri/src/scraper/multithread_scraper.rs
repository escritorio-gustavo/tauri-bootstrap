use itertools::Itertools;
use std::sync::Arc;
use tauri::{State, Window};

use super::{chunk_scraper::scrape_chunk, CONCURRENCY_LIMIT};
use crate::BrowserManagerState;

pub async fn multithread_scrape(
    window: Arc<Window>,
    browser_manager_state: State<'_, BrowserManagerState>,
    tokens: Box<[Arc<str>]>,
) {
    let chunk_size = (tokens.len() as f32 / CONCURRENCY_LIMIT as f32).ceil() as usize;
    let mut handles = Vec::with_capacity(chunk_size);

    let chunks = tokens.chunks(chunk_size).map(|x| x.to_vec()).collect_vec();

    for chunk in chunks {
        let state = Arc::clone(&browser_manager_state.browser_manager_mutex);
        let window = Arc::clone(&window);

        handles.push(std::thread::spawn(move || {
            tauri::async_runtime::block_on(scrape_chunk(window, state, chunk)).unwrap()
        }));
    }

    for handle in handles {
        _ = handle.join();
    }
}
