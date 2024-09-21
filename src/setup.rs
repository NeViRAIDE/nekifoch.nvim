//! This module defines the configuration structure and logic for the Nekifoch plugin.
//!
//! The `Config` struct stores user-defined settings for interacting with Kitty terminal,
//! including the border style for floating windows and the path to the Kitty configuration file.
//!
//! This module also provides functionality for reading these settings from a Neovim `Dictionary`,
//! which allows the plugin to be dynamically configured through Lua scripts (e.g., with `lazy.nvim`).
//!
//! # Configuration Fields
//!
//! The `Config` struct supports the following fields:
//!
//! - `border`: Defines the border style for floating windows in Neovim. Accepts values like `"single"`, `"double"`, etc.
//!   If no value is provided, the default is `"single"`.
//! - `kitty_conf_path`: Specifies the path to the Kitty configuration file. The default value is `"~/.config/kitty/kitty.conf"`.
//!
//! # Usage (in Lua with `lazy.nvim`)
//!
//! Users can define these configuration options in their Lua configuration for Neovim using `lazy.nvim` as shown below:
//!
//! ```lua
//! require('lazy').setup({
//!     {
//!         'NeViRAIDE/nekifoch.nvim',
//!         opts = {
//!             borders = "double",  -- Sets the border style for floating windows
//!             kitty_conf_path = "/custom/path/to/kitty.conf",  -- Specifies a custom Kitty config file path
//!         },
//!     }
//! })
//! ```
//!
//! The `opts` table is passed from Lua and used to create a `Config` instance through the `from_dict` function,
//! allowing the plugin to be reconfigured dynamically.

use nvim_oxi::{conversion::FromObject, Dictionary};

/// A configuration structure for storing Kitty-related settings.
///
/// The `Config` struct holds settings related to Kitty terminal configuration, including the border style
/// for floating windows and the path to the Kitty configuration file. These settings are typically provided
/// from a Lua configuration in Neovim (e.g., using `lazy.nvim`).
///
/// # Fields
///
/// * `border` - The style of the window border (e.g., "single", "double"). Defaults to `"single"`.
/// * `kitty_conf_path` - The path to the Kitty configuration file. Defaults to `"~/.config/kitty/kitty.conf"`.
#[derive(Debug, Default)]
pub struct Config {
    pub border: String,
    pub kitty_conf_path: String,
}

impl Config {
    /// Creates a `Config` instance from a Neovim `Dictionary` of options.
    ///
    /// This function reads the provided `Dictionary` and extracts two keys:
    /// - `"borders"`: The border style for floating windows.
    /// - `"kitty_conf_path"`: The path to the Kitty configuration file.
    ///
    /// If any of the keys are missing, the function will use default values:
    /// - `"single"` for `border`
    /// - `"~/.config/kitty/kitty.conf"` for `kitty_conf_path`
    ///
    /// # Arguments
    ///
    /// * `options` - A `Dictionary` containing configuration options passed from Neovim.
    ///
    /// # Returns
    ///
    /// Returns a `Config` struct populated with the values from the dictionary or the default values if
    /// certain keys are missing.
    ///
    /// # Example (in Lua for `lazy.nvim`)
    ///
    /// ```lua
    /// require('lazy').setup({
    ///     {
    ///         'NeViRAIDE/nekifoch.nvim',
    ///         opts = {
    ///             borders = "double",
    ///             kitty_conf_path = "/custom/path/to/kitty.conf",
    ///         }
    ///     }
    /// })
    /// ```
    ///
    /// In this example, the `opts` table defines the border style as `"double"` and specifies a custom path
    /// to the Kitty configuration file.
    pub fn from_dict(options: Dictionary) -> Self {
        Config {
            border: options
                .get("borders")
                .and_then(|border_obj| String::from_object(border_obj.clone()).ok())
                .unwrap_or_else(|| "single".to_string()),

            kitty_conf_path: options
                .get("kitty_conf_path")
                .and_then(|path_obj| String::from_object(path_obj.clone()).ok())
                .unwrap_or_else(|| "~/.config/kitty/kitty.conf".to_string()),
        }
    }
}
