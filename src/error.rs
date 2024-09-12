use std::io::Error as IoError;

use nvim_oxi::{api::Error as OxiApiError, Error as OxiError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    #[error("Neovim API error: {0}")]
    Api(#[from] OxiApiError),
    #[error("Custom error: {0}")]
    Custom(String),
}

impl From<PluginError> for OxiError {
    fn from(err: PluginError) -> Self {
        OxiError::Api(OxiApiError::Other(format!("{}", err)))
    }
}
