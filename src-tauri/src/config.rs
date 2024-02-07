use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Config {
    pub url: Box<str>,
}

#[derive(serde::Deserialize)]
pub struct TempConfig<'a> {
    url: &'a str
}

impl<'a> From<TempConfig<'a>> for Config {
    fn from(value: TempConfig<'a>) -> Self {
        Self {
            url: value.url.into()
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_json = include_str!("./config/config.json");
        let data: TempConfig =
            serde_json::from_str(config_json).expect("Failed to parse configuration.");

        data.into()
    };
}
