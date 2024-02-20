use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub url: Box<str>,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_json = include_str!("./config/config.json");
        serde_json::from_str(config_json).expect("Failed to parse configuration.")
    };
}
