use std::sync::Arc;

use chromiumoxide::Page;

use crate::prelude::*;

// If the page contains captcha, uncomment this code
// use captcha_oxide::CaptchaSolver;
// use lazy_static::lazy_static;
// lazy_static! {
//     static ref CAPTCHA_SOLVER: CaptchaSolver =
//         CaptchaSolver::new(CONFIG.two_captcha_api_key.as_ref());
// }

pub(super) async fn scrape_document(page: Arc<Page>, search: &str) -> Result<()> {
    Ok(())
}
