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
    api::{create_namespace, err_writeln, out_write},
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
    pub float_window: FloatWindow,
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
            Command::MainMenu => self.float_window.menu_win(&self.config),
            Command::SizeUp => self.size_up(),
            Command::SizeDown => self.size_down(),
            Command::Close => self.float_window.close_win(),
            Command::Check => self.get_current_font(),
            Command::FCheck => self.get_current_font_window(),
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
            Command::FList => self.get_fonts_list_window(),
        }
    }

    fn get_fonts_list_window(&mut self) -> OxiResult<()> {
        let installed_fonts = Utils::list_installed_fonts();
        let compatible = Utils::compare_fonts_with_kitty_list_fonts(installed_fonts);

        let mut fonts: Vec<String> = compatible.values().cloned().collect();
        fonts.sort();

        let content = Utils::format_fonts_in_columns(fonts);

        let window_height = content.lines().count();

        self.float_window
            .f_list_win(&self.config, " Available fonts ", content, window_height)?;

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

    fn handle_font_command<T>(
        &mut self,
        arg: Option<&str>,
        parse_arg: fn(&str) -> Result<T, String>,
        command: fn(&Config, T) -> OxiResult<()>,
        display_window: fn(&mut FloatWindow, &Config) -> OxiResult<()>,
    ) -> OxiResult<()> {
        if let Some(arg_value) = arg {
            match parse_arg(arg_value) {
                Ok(parsed_value) => {
                    command(&self.config, parsed_value)?;
                    out_write(NvimString::from(format!(
                        "Command executed with {}",
                        arg_value
                    )));
                }
                Err(err) => err_writeln(&format!("Error parsing argument: {}", err)),
            }
        } else {
            display_window(&mut self.float_window, &self.config)?;
        }
        Ok(())
    }

    fn get_current_font_window(&mut self) -> OxiResult<()> {
        let current = Utils::get(&self.config)?;

        let content = [
            format!("Family: {}", current["font"]),
            format!("Size:   {}", current["size"]),
        ];

        let binding = content.join("\n");
        let content = Some(binding.as_str());

        self.float_window
            .f_check_win(&self.config, " Current Font Info ", content, 2)
    }

    fn set_font_family(&mut self, arg: Option<&str>) -> OxiResult<()> {
        self.handle_font_command(
            arg,
            |s| Ok(s.to_string()),
            |config, font_family| Utils::replace_font_family(config, &font_family),
            |float_window, config| {
                let installed_fonts = Utils::list_installed_fonts();
                let mut compatible: Vec<String> =
                    Utils::compare_fonts_with_kitty_list_fonts(installed_fonts)
                        .values()
                        .cloned()
                        .collect();
                compatible.sort();

                let fonts = Utils::get(config)?;
                let current_font = fonts["font"].clone();

                float_window.f_family_win(
                    config,
                    " Choose font family ",
                    compatible,
                    10,
                    &current_font,
                )
            },
        )
    }

    fn set_font_size(&mut self, arg: Option<&str>) -> OxiResult<()> {
        self.handle_font_command(
            arg,
            |s| {
                s.parse::<f32>()
                    .map_err(|_| "Invalid size format".to_string())
            },
            Utils::replace_font_size,
            |float_window, config| {
                let fonts = Utils::get(config)?;
                if let Some(current_size_str) = fonts.get("size") {
                    if let Ok(current_size) = current_size_str.parse::<f32>() {
                        float_window.f_size_win(config, " Change font size ", current_size)
                    } else {
                        err_writeln("Invalid current font size in config.");
                        Ok(())
                    }
                } else {
                    err_writeln("Current font size not found.");
                    Ok(())
                }
            },
        )
    }

    pub fn size_up(&mut self) -> OxiResult<()> {
        let fonts = Utils::get(&self.config)?;

        if let Some(size_str) = fonts.get("size") {
            if let Ok(current_size) = size_str.parse::<f32>() {
                let new_size = current_size + 0.5;
                self.set_font_size(Some(&new_size.to_string()))?;
                self.update_size_display(new_size)?;
            } else {
                err_writeln("Invalid font size found in the configuration file.");
            }
        } else {
            err_writeln("Font size not found in the configuration.");
        }

        Ok(())
    }

    pub fn size_down(&mut self) -> OxiResult<()> {
        let fonts = Utils::get(&self.config)?;

        if let Some(size_str) = fonts.get("size") {
            if let Ok(current_size) = size_str.parse::<f32>() {
                let new_size = current_size - 0.5;
                self.set_font_size(Some(&new_size.to_string()))?;
                self.update_size_display(new_size)?;
            } else {
                err_writeln("Invalid font size found in the configuration file.");
            }
        } else {
            err_writeln("Font size not found in the configuration.");
        }

        Ok(())
    }

    fn update_size_display(&mut self, new_size: f32) -> OxiResult<()> {
        if let Some(window) = &self.float_window.window {
            let content = format!("\t\t\t\t\nCurrent size: [ {} ]\n\t\t\t\t", new_size);
            let mut buf = window.get_buf()?;
            BufferManager::set_buffer_content(&mut buf, &content)?;

            let ns_id = create_namespace("font_size_namespace");
            buf.add_highlight(ns_id, "Comment", 1, 0..13)?;
        } else {
            err_writeln("Window is not open.");
        }

        Ok(())
    }
}
