use std::io::Error as IoError;

use nvim_oxi::{api::Error as OxiApiError, Error as OxiError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    #[error("Custom error: {0}")]
    Custom(String),
}

impl From<ConfigError> for OxiError {
    fn from(err: ConfigError) -> Self {
        OxiError::Api(OxiApiError::Other(format!("{}", err)))
    }
}
