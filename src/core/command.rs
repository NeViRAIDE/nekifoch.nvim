//! This module defines commands for interacting with the Nekifoch plugin.
//!
//! The primary responsibilities of this module include:
//!
//! - Defining the `Command` enum, which represents various commands that can be issued to the plugin.
//! - Providing functions for handling font-related operations such as retrieving a list of fonts.
//!
//! The commands defined in this module are dispatched and handled in the `App` struct from the core module.
//!
//! # Key Components:
//!
//! - `Command`: Enum representing different commands available in the plugin.
//! - `from_str`: Function for parsing a command and its argument from strings.
//! - `get_fonts_list`: Function for retrieving and printing the list of available fonts.

#[derive(Debug)]
pub enum Command {
    MainMenu,
    SizeUp,
    SizeDown,
    Close,
    Check,
    FCheck,
    SetFont(Option<String>),
    SetSize(Option<f32>),
    List,
    FList,
}

/// Parses a command and its argument from strings.
///
/// This function takes a command string and an optional argument, and returns
/// a corresponding `Command` variant if the input is valid. The `set_font` command
/// accepts an optional argument, while other commands may require or ignore arguments.
///
/// # Arguments
///
/// * `cmd` - A string representing the command.
/// * `arg` - An optional argument for the command. For example, a font family name for `set_font`.
///
/// # Returns
///
/// Returns `Some(Command)` if the input matches a known command. Returns `None` if the command is unrecognized.
impl Command {
    pub fn from_str(cmd: &str, arg: Option<&str>) -> Option<Self> {
        match cmd {
            "" => Some(Command::MainMenu),
            "size_up" => Some(Command::SizeUp),
            "size_down" => Some(Command::SizeDown),
            "close" => Some(Command::Close),
            "check" => Some(Command::Check),
            "float_check" => Some(Command::FCheck),
            "set_font" => Some(Command::SetFont(arg.map(|s| s.to_string()))),
            "set_size" => {
                let size = arg.and_then(|s| s.parse::<f32>().ok());
                Some(Command::SetSize(size))
            }
            "list" => Some(Command::List),
            "float_list" => Some(Command::FList),
            _ => None,
        }
    }
}
