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

use nvim_oxi::{
    api::{err_writeln, out_write},
    print, Dictionary, Result as OxiResult, String as NvimString,
};

use crate::{setup::Config, utils::Utils};

use buffer::BufferManager;
use command::{get_fonts_list, Command};
use window::FloatWindow;

mod buffer;
pub mod command;
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
    pub fn handle_command(&mut self, cmd: Command) -> OxiResult<()> {
        match cmd {
            Command::MainMenu => self.show_main_menu(),
            Command::Close => self.float_window.close(),
            Command::Check => self.get_current_font(),
            Command::SetFont(font) => {
                if font.is_some() {
                    self.set_font_family(font.as_deref())
                } else {
                    self.set_font_family(None)
                }
            }
            Command::SetSize(size) => {
                if let Some(size_value) = size {
                    self.set_font_size(Some(&size_value.to_string()))
                } else {
                    self.set_font_size(None)
                }
            }
            Command::List => get_fonts_list(),
        }
    }

    fn show_main_menu(&mut self) -> OxiResult<()> {
        let menu_options = vec![
            "Check current font".to_string(),
            "Set font family".to_string(),
            "Set font size".to_string(),
            "Show installed fonts".to_string(),
        ];

        self.float_window
            .open(&self.config, " NeKiFoCh ", menu_options)?;

        if let Some(window) = &self.float_window.window {
            BufferManager::configure_buffer(window)?;
            let mut buf = window.get_buf()?;
            mapping::set_keymaps_for_menu(&mut buf)?;
        }

        Ok(())
    }

    /// Retrieves and displays the current font family and size from the Kitty terminal configuration.
    ///
    /// This method queries the current font settings stored in the Kitty terminal configuration
    /// and prints the font family and size in the Neovim output. The font information is fetched
    /// using a utility function from the `Utils` module, which interacts with the configuration
    /// specified in the `config` field of the `App` structure.
    ///
    /// # Returns
    ///
    /// Returns an `OxiResult<()>` to indicate success or failure. If an error occurs while fetching
    /// the font data, the error will be propagated up to the caller.
    ///
    /// # Errors
    ///
    /// This function returns an error if the `Utils::get` function fails to retrieve the font
    /// configuration from Kitty.
    fn get_current_font(&mut self) -> OxiResult<()> {
        let fonts = Utils::get(&self.config)?;
        print!(
            "\nFont family: {:?}\nFont size: {:?}\n",
            fonts["font"], fonts["size"]
        );
        Ok(())
    }

    /// Sets the font family in the Kitty terminal configuration or opens a floating window to select a font.
    ///
    /// If a font family is provided as an argument, it will update the font in the Kitty terminal configuration
    /// using the `Utils::replace_font_family` function. If no argument is provided, a floating window with a list of
    /// available fonts will be opened, allowing the user to select one. The selected font will then be applied.
    ///
    /// # Arguments
    ///
    /// * `arg` - An optional string containing the font family to set. If `None`, a font selection window will be displayed.
    ///
    /// # Returns
    ///
    /// Returns an `OxiResult<()>` to indicate success or failure. If the font family cannot be updated or the floating
    /// window cannot be opened, an error will be propagated.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The font family cannot be replaced.
    /// - The floating window fails to open.
    /// - The window buffer cannot be configured.
    fn set_font_family(&mut self, arg: Option<&str>) -> OxiResult<()> {
        if let Some(font_family) = arg {
            Utils::replace_font_family(&self.config, font_family)?;
            out_write(NvimString::from(format!(
                "Font family set to {}",
                font_family
            )));
        } else {
            let installed_fonts = Utils::list_installed_fonts();
            let mut compatible: Vec<String> =
                Utils::compare_fonts_with_kitty_list_fonts(installed_fonts)
                    .values()
                    .cloned()
                    .collect();
            compatible.sort();

            if let Err(err) =
                self.float_window
                    .open(&self.config, " Choose font family ", compatible)
            {
                out_write(NvimString::from(format!("Error opening window: {}", err)));
            }

            if let Some(window) = &self.float_window.window {
                BufferManager::configure_buffer(window)?;
            } else {
                err_writeln("Window is not open.");
            }
        }
        Ok(())
    }

    /// Sets the font size in the Kitty terminal configuration.
    ///
    /// This method updates the font size in the Kitty terminal configuration based on the provided argument.
    /// The argument should be a string that can be parsed into a floating-point number (i.e., the font size).
    /// If the argument is missing or invalid, an error message will be displayed in Neovim.
    ///
    /// # Arguments
    ///
    /// * `arg` - An optional string containing the font size to set. If `None`, an error message will be printed.
    ///
    /// # Returns
    ///
    /// Returns an `OxiResult<()>` to indicate success or failure. If an invalid font size is provided, an error message
    /// will be printed instead.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The font size cannot be parsed as a floating-point number.
    /// - The font size argument is missing.
    fn set_font_size(&mut self, arg: Option<&str>) -> OxiResult<()> {
        if let Some(size_str) = arg {
            if let Ok(size) = size_str.parse::<f32>() {
                Utils::replace_font_size(&self.config, size)?;
                out_write(NvimString::from(format!("Font size set to {}", size)));
            } else {
                err_writeln("Invalid font size argument for set_size action");
            }
        } else if let Err(err) = self
            .float_window
            .open_for_input(&self.config, " Enter font size ")
        {
            out_write(NvimString::from(format!(
                "Error opening input window: {}",
                err
            )));
        }

        if let Some(window) = &self.float_window.window {
            BufferManager::configure_buffer(window)?;
        } else {
            err_writeln("Window is not open.");
        }

        Ok(())
    }
}
