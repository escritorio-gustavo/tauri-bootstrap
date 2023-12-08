use std::sync::Arc;

use captcha_oxide::{captcha_types::normal_captcha::NormalCaptcha, CaptchaSolver, CaptchaTask};
use chromiumoxide::{
    cdp::browser_protocol::{
        browser::{SetDownloadBehaviorBehavior, SetDownloadBehaviorParams},
        network::{DeleteCookiesParams, EventResponseReceived},
    },
    Page,
};
use futures::StreamExt;
use lazy_static::lazy_static;

use crate::{model::search::Search, prelude::*};

lazy_static! {
    static ref CAPTCHA_SOLVER: CaptchaSolver =
        CaptchaSolver::new(CONFIG.two_captcha_api_key.as_ref());
}

pub(super) async fn scrape_document(page: Arc<Page>, search: &Search) -> Result<()> {
}
