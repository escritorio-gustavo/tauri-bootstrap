use std::{path::PathBuf, str::FromStr};

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Config {}

#[derive(serde::Deserialize)]
pub struct TempConfig<'a> {}

impl<'a> From<TempConfig<'a>> for Config {
    fn from(value: TempConfig<'a>) -> Self {
        Self {}
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_json = include_str!("config.json");
        let data: TempConfig =
            serde_json::from_str(config_json).expect("Failed to parse configuration.");

        data.into()
    };
}
