//! This module defines the core functionality of the Nekifoch plugin.
//!
//! The core responsibilities of this module include:
//!
//! - Managing the configuration options for interacting with Kitty, such as setting font family and size.
//! - Handling Neovim commands that allow users to configure their Kitty terminal directly from Neovim.
//! - Managing floating windows for displaying font-related information inside Neovim.
//!
//! # Key Components:
//!
//! - `App`: The main struct that holds the plugin's state and configuration. It handles command dispatch and
//!   interacts with the window manager to display floating windows.
//! - Commands such as `set_font`, `set_size`, `check`, and `list`, which are integrated into Neovim for seamless
//!   user interaction with the Kitty terminal.
//!
//! # Commands Available:
//!
//! The following commands can be executed in Neovim to interact with the plugin:
//!
//! - `:Nekifoch set_font <font_family>`: Set the Kitty terminal's font family.
//! - `:Nekifoch set_size <font_size>`: Set the font size in the Kitty terminal.
//! - `:Nekifoch check`: Display the current font family.
//! - `:Nekifoch list`: List all available fonts that can be used with Kitty.
//! - `:Nekifoch close`: Close the floating window used to display font information.

use nvim_oxi::{api::err_writeln, Dictionary, Result as OxiResult};

use crate::setup::Config;

use commands::{get_current_font, get_fonts_list, set_font_family, set_font_size};
use window::FloatWindow;

mod buffer;
mod commands;
pub mod completion;
mod mapping;
mod window;

/// The core structure that holds the plugin's state and handles commands.
///
/// The `App` struct maintains the configuration (`Config`) and manages the floating window used for displaying
/// font-related information. It supports a set of commands that allow users to interact with Kitty directly
/// from Neovim.
///
/// # Commands Supported:
///
/// - `close`: Close the floating window.
/// - `check`: Check and display the current font family.
/// - `set_font <font_family>`: Set the font family to the specified value.
/// - `set_size <font_size>`: Set the font size to the specified value.
/// - `list`: List all available fonts.
///
/// These commands can be used through the `:Nekifoch` command interface in Neovim.
#[derive(Debug)]
pub struct App {
    config: Config,
    float_window: FloatWindow,
}

impl App {
    /// Creates a new `App` instance with the provided configuration.
    ///
    /// This function initializes the application state with the specified `Config`.
    pub fn new(config: Config) -> Self {
        App {
            config,
            float_window: FloatWindow::new(),
        }
    }

    /// Sets up the application with the provided options from a `Dictionary`.
    ///
    /// This function allows the plugin to be reconfigured dynamically, using
    /// a dictionary of options passed from Neovim.
    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        let config = Config::from_dict(dict);
        self.config = config;
        Ok(())
    }

    /// Handles commands issued to the plugin.
    ///
    /// Based on the command and argument passed, the corresponding action (such as
    /// setting the font or closing the window) is performed.
    pub fn handle_command(&mut self, cmd: &str, arg: Option<&str>) -> OxiResult<()> {
        match cmd {
            "close" => self.float_window.close(),
            "check" => get_current_font(self),
            "set_font" => set_font_family(self, arg),
            "set_size" => set_font_size(self, arg),
            "list" => get_fonts_list(),
            _ => {
                err_writeln(&format!("Unknown command: {}", cmd));
                Ok(())
            }
        }
    }
}
