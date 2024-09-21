//! This module defines error handling for the Nekifoch plugin.
//!
//! It introduces a custom error type `PluginError` that encapsulates various types of errors that can occur in the plugin,
//! such as I/O errors, Neovim API errors, and custom errors. This allows the plugin to handle and propagate errors
//! in a consistent and structured way.
//!
//! The `PluginError` enum supports conversion from standard I/O errors and Neovim API errors, which is facilitated by the
//! `thiserror` crate. Additionally, this module provides an implementation of conversion from `PluginError` to `OxiError`
//! for compatibility with the `nvim-oxi` API.

use nvim_oxi::{api::Error as OxiApiError, Error as OxiError};
use std::io::Error as IoError;
use thiserror::Error;

/// A custom error type for the Nekifoch plugin.
///
/// The `PluginError` enum handles different error scenarios that can occur in the plugin, such as:
/// - I/O errors (for file operations)
/// - Neovim API errors (related to Neovim interactions)
/// - Custom errors (for any other specific cases where a custom error message is needed)
///
/// # Variants
///
/// * `Io`: Represents an I/O error, commonly used for reading or writing files. It wraps the standard `std::io::Error`.
/// * `Api`: Represents an error from the Neovim API, wrapping `nvim_oxi::api::Error`.
/// * `Custom`: A custom error that stores a string message for flexibility in error reporting.
///
/// # Example
///
/// ```rust
/// use std::fs::File;
/// use crate::error::PluginError;
///
/// fn read_file(path: &str) -> Result<String, PluginError> {
///     let file = File::open(path)?;
///     // Other file processing...
///     Ok("File content".to_string())
/// }
/// ```
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
    /// Converts a `PluginError` into a `nvim_oxi::Error`.
    ///
    /// This allows the `PluginError` to be returned where an `OxiError` is expected, ensuring compatibility
    /// with the Neovim API. It wraps the `PluginError` as an `OxiApiError::Other` variant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::error::PluginError;
    /// use nvim_oxi::Error as OxiError;
    ///
    /// fn example() -> Result<(), OxiError> {
    ///     let error: PluginError = PluginError::Custom("Something went wrong".to_string());
    ///     Err(OxiError::from(error))
    /// }
    /// ```
    fn from(err: PluginError) -> Self {
        OxiError::Api(OxiApiError::Other(format!("{}", err)))
    }
}
