use chromiumoxide::{error::CdpError, fetcher::FetcherError};
use tokio::sync::SetError;

#[derive(thiserror::Error, Debug, serde::Serialize)]
pub enum Error {
    #[error("Could not find app's local data directory")]
    AppLocalDataDir,

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    OnceCellSetError(#[from] SetError<std::path::PathBuf>),

    #[error("EXECUTABLE_PATH has not been set")]
    ExecutablePathNotSet,

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    BrowserFetcher(#[from] FetcherError),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    Cdp(#[from] CdpError),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    DbConnection(#[from] sqlx::error::Error),

    #[error("Failed to launch browser")]
    BrowserLaunch,

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    JsonParse(#[from] serde_json::Error),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    Captcha(#[from] captcha_oxide::Error),

    #[error("The solution provided by 2captcha was wrong")]
    IncorrectCaptcha,

    #[error("{0}")]
    UnknownScrapeError(String),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    UrlParse(#[from] url::ParseError),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    DateParse(#[from] chrono::ParseError),

    #[error("Failed to parse (0) into TipoParte")]
    TipoParteParse(String),

    #[error(transparent)]
    #[serde(serialize_with = "serialize_error")]
    Xlsx(#[from] rust_xlsxwriter::XlsxError),

    #[error("Bot detected")]
    BotDetected,
}

fn serialize_error<S: serde::Serializer>(
    v: &impl std::error::Error,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(v.to_string().as_ref())
}
