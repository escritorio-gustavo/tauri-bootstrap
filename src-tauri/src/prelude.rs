pub use crate::config::CONFIG;
pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;
