use chromiumoxide::cdp::browser_protocol::{
    fetch::{
        ContinueRequestParams, EnableParams, EventRequestPaused, FailRequestParams, RequestPattern,
        RequestStage,
    },
    network::{ErrorReason, ResourceType},
};
use futures::StreamExt;
use std::sync::Arc;
use tauri::{async_runtime::Mutex, Manager, Window};

use super::{browser_factory::create_browser, document_scraper::scrape_document};
use crate::{browser_manager::BrowserManager, prelude::*, DATA_SET};

pub(super) async fn scrape_chunk(
    window: Arc<Window>,
    state: Arc<Mutex<BrowserManager>>,
    chunk: Vec<Arc<str>>,
) -> Result<()> {
    let (mut browser, mut handler) = create_browser().await?;

    let handle = tauri::async_runtime::spawn(async move {
        while let Some(event) = handler.next().await {
            if event.is_err() {
                break;
            }
        }
    });

    let pid = browser.get_mut_child().unwrap().inner.id().unwrap();

    browser_spawned(&state, pid).await;

    let close_page = browser.new_page("about:blank").await?;

    let page = Arc::new(close_page.clone());
    page.enable_stealth_mode().await?;
    page.execute(EnableParams {
        patterns: Some(vec![RequestPattern {
            url_pattern: "*".to_string().into(),
            resource_type: None,
            request_stage: Some(RequestStage::Request),
        }]),
        handle_auth_requests: None,
    })
    .await?;

    let intercept_page = page.clone();

    let mut request_paused = page.event_listener::<EventRequestPaused>().await?;

    let intercept_handle = tauri::async_runtime::spawn(async move {
        while let Some(event) = request_paused.next().await {
            match event.resource_type {
                ResourceType::Stylesheet
                | ResourceType::Media
                | ResourceType::Font
                | ResourceType::TextTrack => {
                    _ = intercept_page
                        .execute(FailRequestParams {
                            error_reason: ErrorReason::Aborted,
                            request_id: event.request_id.clone(),
                        })
                        .await;
                }
                _ => {
                    _ = intercept_page
                        .execute(ContinueRequestParams::new(event.request_id.clone()))
                        .await;
                }
            }
        }
    });

    for search in chunk {
        loop {
            let search = Arc::clone(&search);
            let scrape_result = scrape_document(Arc::clone(&page), &search).await;

            match scrape_result {
                Err(Error::IncorrectCaptcha | Error::BotDetected | Error::Captcha(_)) => continue,
                Err(Error::Cdp(_)) => break,
                x => {
                    match x {
                        Ok(x) => {
                            let mut w_lock = DATA_SET.write().await;

                            w_lock.push(Arc::new(x));
                            drop(w_lock);

                            window.emit_all("result", search).unwrap();

                            // Save to DB
                        }
                        Err(e) => {
                            window.emit_all("error", (search, e.to_string())).unwrap();
                        }
                    }

                    break;
                }
            }
        }
    }

    _ = close_page.close().await;

    browser.close().await?;
    browser.wait().await?;
    _ = handle.await;
    _ = intercept_handle.await;

    browser_killed(&state, pid).await;

    Ok(())
}

async fn browser_spawned(state: &Arc<Mutex<BrowserManager>>, pid: u32) {
    let mut state = state.lock().await;
    state.push(pid);
}

async fn browser_killed(state: &Arc<Mutex<BrowserManager>>, pid: u32) {
    let mut state = state.lock().await;
    state.pop(pid);
}
