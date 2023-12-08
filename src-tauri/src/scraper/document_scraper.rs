use std::sync::Arc;

use captcha_oxide::CaptchaSolver;
use chromiumoxide::Page;
use lazy_static::lazy_static;

use crate::prelude::*;

lazy_static! {
    static ref CAPTCHA_SOLVER: CaptchaSolver =
        CaptchaSolver::new(CONFIG.two_captcha_api_key.as_ref());
}

pub(super) async fn scrape_document(page: Arc<Page>, search: &str) -> Result<()> {
    Ok(())
}
