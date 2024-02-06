macro_rules! intercept {
    ($page: expr) => {{
        use chromiumoxide::cdp::browser_protocol::{
            fetch::{
                ContinueRequestParams, EnableParams, EventRequestPaused, FailRequestParams,
                RequestPattern, RequestStage,
            },
            network::{ErrorReason, ResourceType},
        };
        use futures::StreamExt;

        $page.enable_stealth_mode().await?;
        $page
            .execute(EnableParams {
                patterns: Some(vec![RequestPattern {
                    url_pattern: "*".to_string().into(),
                    resource_type: None,
                    request_stage: Some(RequestStage::Request),
                }]),
                handle_auth_requests: None,
            })
            .await?;

        let intercept_page = Arc::clone(&$page);
        let mut listener = intercept_page
            .event_listener::<EventRequestPaused>()
            .await?;
        tauri::async_runtime::spawn(async move {
            while let Some(event) = listener.next().await {
                match event.resource_type {
                    ResourceType::Stylesheet
                    | ResourceType::Image
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
        })
    }};
}

pub(super) use intercept;

