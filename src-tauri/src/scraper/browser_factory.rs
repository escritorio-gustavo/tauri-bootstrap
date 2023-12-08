use rand::RngCore;
use std::path::Path;

use chromiumoxide::{Browser, BrowserConfig, BrowserFetcher, BrowserFetcherOptions, Handler};

use crate::prelude::*;

pub(super) async fn create_browser() -> Result<(Browser, Handler)> {
    let download_path = Path::new(r".\download");
    let _ = std::fs::create_dir_all(download_path);

    let fetcher_config = BrowserFetcherOptions::builder()
        .with_path(download_path)
        .build()?;

    let fetcher = BrowserFetcher::new(fetcher_config);

    let info = fetcher.fetch().await?;

    let number = rand::thread_rng().next_u64();
    let config_builder = BrowserConfig::builder()
        .chrome_executable(info.executable_path)
        .launch_timeout(std::time::Duration::from_secs(20))
        .no_sandbox()
        .request_timeout(std::time::Duration::from_secs(3_600))
        .enable_request_intercept()
        .user_data_dir(format!(r"..\..\data-dir-{number}"));
    #[cfg(dev)]
    let config_builder = config_builder.with_head();

    let config = config_builder.build().map_err(|_| Error::BrowserLaunch)?;

    let (browser, handler) = Browser::launch(config)
        .await
        .map_err(|_| Error::BrowserLaunch)?;

    Ok((browser, handler))
}
