use crate::config::CONFIG;
use lazy_static::lazy_static;
use std::str::FromStr;
use url::Url;

mod browser_factory;
mod chunk_scraper;
mod document_scraper;
mod intercept;
mod multithread_scraper;
mod spawn_waiter;

#[cfg(dev)]
pub(super) const CONCURRENCY_LIMIT: usize = 1;

#[cfg(not(dev))]
pub(super) const CONCURRENCY_LIMIT: usize = 15;

lazy_static! {
    pub(super) static ref URL: Url = Url::from_str(&CONFIG.url).unwrap();
}

pub use multithread_scraper::multithread_scrape as scrape;
