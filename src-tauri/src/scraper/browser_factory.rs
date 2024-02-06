use chromiumoxide::{Browser, BrowserConfig, Handler};
use rand::RngCore;

use crate::{prelude::*, BROWSER_PATH, EXECUTABLE_PATH};

pub(super) async fn create_browser() -> Result<(Browser, Handler)> {
    let number = rand::thread_rng().next_u64();
    let config_builder = BrowserConfig::builder()
        .chrome_executable(EXECUTABLE_PATH.get().unwrap())
        .launch_timeout(std::time::Duration::from_secs(20))
        .no_sandbox()
        .request_timeout(std::time::Duration::from_secs(3_600))
        .enable_request_intercept()
        .user_data_dir(BROWSER_PATH.join(format!(r"data-dir-{number}")));

    #[cfg(dev)]
    let config_builder = config_builder.with_head();

    let config = config_builder.build().map_err(|_| Error::BrowserLaunch)?;

    let (browser, handler) = Browser::launch(config)
        .await
        .map_err(|_| Error::BrowserLaunch)?;

    Ok((browser, handler))
}
