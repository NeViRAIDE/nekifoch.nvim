use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

use dirs::home_dir;
use nvim_oxi::Result as OxiResult;
use regex::Regex;
use serde_json::Value;

use crate::{error::PluginError, Config};

/// A utility module providing common operations for handling font configurations and file I/O.
///
/// This module includes various helper functions for:
/// - Reading and writing configuration files.
/// - Modifying font settings (family and size) in configuration files.
/// - Listing installed fonts and comparing them with fonts supported by the Kitty terminal.
/// - Caching installed fonts for performance improvements.
///
/// The core functionalities provided by this module are designed to work specifically
/// with the configuration of the Kitty terminal, allowing the user to dynamically
/// change the font settings and reload the terminal to apply the changes.
///
/// Example:
/// ```rust
/// use crate::utils::Utils;
///
/// let config = Config::default();
/// let fonts = Utils::list_installed_fonts();
/// println!("Installed fonts: {:?}", fonts);
/// ```
pub struct Utils;

/// A lazily initialized, thread-safe cache of installed fonts.
///
/// `FONT_CACHE` stores a `HashMap` where the keys are font names without spaces, and the values are the original font names.
/// This cache is populated the first time fonts are listed using the `Utils::get_cached_installed_fonts` function.
///
/// The use of `OnceLock` ensures that the font list is only initialized once and is safe to access from multiple threads.
/// This prevents unnecessary calls to the system to list installed fonts, improving performance when accessing font data multiple times.
///
/// Example:
/// ```rust
/// let cached_fonts = Utils::get_cached_installed_fonts();
/// println!("Cached fonts: {:?}", cached_fonts);
/// ```
static FONT_CACHE: OnceLock<HashMap<String, String>> = OnceLock::new();

impl Utils {
    /// Reads the content of a file into a `String`.
    ///
    /// This function takes a file path as input, opens the file, reads its entire content,
    /// and returns it as a `String`. If the file cannot be opened or read, it returns a `PluginError`.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the file to be read.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<String, PluginError>`. On success, it returns the file content as a `String`.
    /// On failure, it returns a `PluginError` indicating the type of error encountered (e.g., file not found, read error).
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file cannot be opened (e.g., the file does not exist or there are insufficient permissions).
    /// - The file content cannot be read due to I/O errors.
    ///
    /// # Example
    ///
    /// ```rust
    /// let content = Utils::read_file_to_string("/path/to/file.conf")
    ///     .expect("Failed to read the file");
    /// println!("File content: {}", content);
    /// ```
    fn read_file_to_string(file_path: &str) -> Result<String, PluginError> {
        let file = File::open(file_path).map_err(PluginError::from)?;
        let mut content = String::new();
        BufReader::new(file)
            .read_to_string(&mut content)
            .map_err(PluginError::from)?;
        Ok(content)
    }

    /// Writes a `String` to a file.
    ///
    /// This function creates or overwrites the file at the specified path and writes the provided string content to it.
    /// If the file cannot be created or written to, it returns a `PluginError`.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the file where the content will be written.
    /// * `content` - A string slice that contains the data to be written to the file.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<(), PluginError>`. On success, it returns `Ok(())`.
    /// On failure, it returns a `PluginError` indicating the type of error encountered (e.g., I/O error).
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file cannot be created or opened for writing (e.g., due to insufficient permissions or invalid path).
    /// - There is an I/O error while writing to the file.
    ///
    /// # Example
    ///
    /// ```rust
    /// Utils::write_string_to_file("/path/to/file.conf", "This is the file content")
    ///     .expect("Failed to write to the file");
    /// ```
    fn write_string_to_file(file_path: &str, content: &str) -> Result<(), PluginError> {
        let mut file = BufWriter::new(File::create(file_path).map_err(PluginError::from)?);
        file.write_all(content.as_bytes())
            .map_err(PluginError::from)?;
        Ok(())
    }

    /// Modifies the content of a file using a custom modification function.
    ///
    /// This function reads the content of a file at the specified path, passes the content to the provided
    /// modification function, and writes the modified content back to the file. It handles reading, modifying,
    /// and writing the file, simplifying file modification operations.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the file to be modified.
    /// * `modify_fn` - A closure or function that takes the file content as a `String` and returns either
    ///   the modified content as a `String` or a `PluginError` in case of a failure.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<(), PluginError>`. On success, it returns `Ok(())`.
    /// On failure, it returns an error that can occur during file reading, modification, or writing.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file cannot be read (e.g., due to file not found or insufficient permissions).
    /// - The provided modification function returns an error.
    /// - The modified content cannot be written back to the file.
    ///
    /// # Example
    ///
    /// ```rust
    /// Utils::modify_file_content("/path/to/file.conf", |content| {
    ///     let modified = content.replace("old_value", "new_value");
    ///     Ok(modified)
    /// }).expect("Failed to modify file content");
    /// ```
    fn modify_file_content<F>(file_path: &str, modify_fn: F) -> OxiResult<()>
    where
        F: FnOnce(String) -> Result<String, PluginError>,
    {
        let content = Self::read_file_to_string(file_path)?;
        let modified_content = modify_fn(content)?;
        Self::write_string_to_file(file_path, &modified_content)?;
        Ok(())
    }

    /// Applies a regular expression replacement to a given string.
    ///
    /// This function takes a string content, a regular expression pattern, and a replacement function.
    /// It applies the regex to the content and replaces matching parts according to the provided replacement function.
    /// If the regex pattern is invalid, it returns a `PluginError`.
    ///
    /// # Arguments
    ///
    /// * `content` - The input string in which the regular expression will be applied.
    /// * `regex_pattern` - A string slice representing the regular expression pattern to search for in the content.
    /// * `replacement` - A closure or function that generates the replacement string for each regex match.
    ///   It takes `regex::Captures` and returns a `String`.
    ///
    /// # Returns
    ///
    /// This function returns a `Result<String, PluginError>`. On success, it returns the modified string.
    /// On failure (e.g., if the regex pattern is invalid), it returns a `PluginError`.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The provided regex pattern is invalid and cannot be compiled.
    ///
    /// # Example
    ///
    /// ```rust
    /// let modified = Utils::apply_regex_replace(
    ///     "font_family old_font",
    ///     r"(?m)^(font_family\s+)(.*)",
    ///     |caps| format!("{}new_font", &caps[1])
    /// ).expect("Failed to apply regex replacement");
    ///
    /// println!("Modified content: {}", modified);
    /// ```
    fn apply_regex_replace(
        content: &str,
        regex_pattern: &str,
        replacement: impl Fn(&regex::Captures) -> String,
    ) -> Result<String, PluginError> {
        let regex =
            Regex::new(regex_pattern).map_err(|_| PluginError::Custom("Invalid regex".into()))?;
        let modified_content = regex.replace_all(content, replacement);
        Ok(modified_content.to_string())
    }

    /// Retrieves the current font configuration from the Kitty terminal configuration file.
    ///
    /// This function reads the Kitty configuration file at the path specified in the provided `Config`,
    /// and extracts the `font_family` and `font_size` settings. It returns these settings in a `HashMap`
    /// where the keys are `"font"` and `"size"`.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `Config` struct that contains the path to the Kitty configuration file.
    ///
    /// # Returns
    ///
    /// This function returns an `OxiResult<HashMap<String, String>>`. On success, the `HashMap` contains
    /// the `font` and `size` settings. If the file does not exist or there is an error reading the file,
    /// the function returns an error wrapped in `OxiResult`.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The Kitty configuration file is not found at the specified path.
    /// - The file cannot be read due to I/O errors.
    /// - The `font_size` setting cannot be parsed as a valid floating-point number.
    ///
    /// # Example
    ///
    /// ```rust
    /// let config = Config {
    ///     kitty_conf_path: "~/.config/kitty/kitty.conf".to_string(),
    /// };
    ///
    /// let font_settings = Utils::get(&config).expect("Failed to get font settings");
    /// println!("Font family: {}", font_settings["font"]);
    /// println!("Font size: {}", font_settings["size"]);
    /// ```
    pub fn get(config: &Config) -> OxiResult<HashMap<String, String>> {
        let config_path = Self::expand_tilde(&config.kitty_conf_path);
        if !Path::new(&config_path).exists() {
            return Err(PluginError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", config_path),
            ))
            .into());
        }

        let content = Self::read_file_to_string(&config_path)?;

        let mut current_font_family: Option<String> = None;
        let mut current_font_size: Option<String> = None;

        for line in content.lines() {
            if line.starts_with("font_family ") {
                current_font_family =
                    Some(line.trim_start_matches("font_family ").trim().to_string());
            } else if line.starts_with("font_size ") {
                let size_str = line.trim_start_matches("font_size ").trim();
                let parsed_size = size_str.parse::<f32>();
                current_font_size = match parsed_size {
                    Ok(_) => Some(size_str.to_string()),
                    Err(_) => None,
                };
            }
        }

        let mut result = HashMap::new();
        result.insert("font".to_string(), current_font_family.unwrap_or_default());
        result.insert(
            "size".to_string(),
            current_font_size.unwrap_or_else(|| "default".to_string()),
        );

        Ok(result)
    }

    /// Retrieves a list of installed fonts on the system.
    ///
    /// This function executes the `fc-list` command to obtain a list of installed fonts available on the system.
    /// It extracts the font family names and returns them as a `Vec<String>`. Duplicate font family names are filtered out.
    ///
    /// This function is intended to be used on systems where `fc-list` is available (typically Linux-based systems).
    ///
    /// # Returns
    ///
    /// This function returns a `Vec<String>` containing the names of installed fonts. The list is filtered
    /// to avoid duplicates, ensuring that each font family appears only once.
    ///
    /// # Panics
    ///
    /// This function will panic if the `fc-list` command fails to execute, as it uses `expect` to handle this case.
    ///
    /// # Example
    ///
    /// ```rust
    /// let fonts = Utils::list_installed_fonts();
    /// println!("Installed fonts: {:?}", fonts);
    /// ```
    ///
    /// # Notes
    ///
    /// The function relies on external system commands, so it is not portable to platforms where `fc-list` is unavailable (e.g., Windows).
    pub fn list_installed_fonts() -> Vec<String> {
        let cmd = "fc-list : family 2>/dev/null | awk -F ',' '{print $1}'";
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        let result = String::from_utf8_lossy(&output.stdout);
        let mut installed_fonts = Vec::new();
        let mut seen_fonts = HashSet::new();

        for font in result.lines() {
            if seen_fonts.insert(font.to_string()) {
                installed_fonts.push(font.to_string());
            }
        }

        installed_fonts
    }

    /// Compares a list of installed fonts with the fonts supported by the Kitty terminal.
    ///
    /// This function takes a list of installed fonts and compares it with the fonts available in the Kitty terminal.
    /// It retrieves the supported fonts from Kitty by executing a Kitty-specific Python command, parses the result,
    /// and returns a `HashMap` where the keys are the installed fonts (with spaces removed) that are compatible with Kitty,
    /// and the values are the original font names.
    ///
    /// # Arguments
    ///
    /// * `installed_fonts` - A `Vec<String>` containing the list of installed fonts on the system.
    ///
    /// # Returns
    ///
    /// This function returns a `HashMap<String, String>`. The keys are the formatted font names (with spaces removed),
    /// and the values are the original font names. Only fonts that are both installed on the system and supported by
    /// Kitty will be included in the result.
    ///
    /// # Panics
    ///
    /// This function will panic if the Kitty command fails to execute, as it uses `expect` to handle this case.
    ///
    /// # Example
    ///
    /// ```rust
    /// let installed_fonts = Utils::list_installed_fonts();
    /// let compatible_fonts = Utils::compare_fonts_with_kitty_list_fonts(installed_fonts);
    /// for (formatted_name, original_name) in compatible_fonts {
    ///     println!("Compatible font: {} ({})", formatted_name, original_name);
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// - This function is intended to work only with systems where Kitty is installed and can execute the `kitty +runpy` command.
    /// - The function relies on external system commands and assumes that Kitty's Python environment is properly configured.
    pub fn compare_fonts_with_kitty_list_fonts(
        installed_fonts: Vec<String>,
    ) -> HashMap<String, String> {
        let cmd = r#"kitty +runpy "from kitty.fonts.common import all_fonts_map; import json; print(json.dumps(all_fonts_map(True), indent=2))" 2>/dev/null"#;
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        let result = String::from_utf8_lossy(&output.stdout);

        let json: Value = serde_json::from_str(&result).unwrap_or(Value::Null);
        let kitty_fonts = Self::extract_fonts_from_json(&json);

        let mut compatible_fonts_map = HashMap::new();

        for font in installed_fonts {
            if kitty_fonts.contains(&font) {
                let formatted_font = font.replace(" ", "");
                compatible_fonts_map.insert(formatted_font, font);
            }
        }

        compatible_fonts_map
    }

    /// Extracts font family names from a Kitty configuration JSON object.
    ///
    /// This function parses a JSON object, typically retrieved from Kitty's font configuration,
    /// and extracts the font family names. It searches for the `family_map` key in the JSON structure and
    /// collects the font family names into a `HashSet<String>`, ensuring that each font is unique.
    ///
    /// # Arguments
    ///
    /// * `json` - A reference to a `serde_json::Value` object containing the JSON data to parse.
    ///   This is expected to be a JSON structure similar to the output of Kitty's `all_fonts_map()` function.
    ///
    /// # Returns
    ///
    /// This function returns a `HashSet<String>` containing the unique font family names found in the JSON object.
    ///
    /// # Example
    ///
    /// ```rust
    /// let json: serde_json::Value = ...; // JSON data from Kitty
    /// let fonts = Utils::extract_fonts_from_json(&json);
    /// println!("Extracted fonts: {:?}", fonts);
    /// ```
    ///
    /// # Notes
    ///
    /// - This function assumes that the `json` input follows the structure expected from Kitty's font data.
    /// - It safely handles cases where `family_map` is missing or incorrectly formatted by returning an empty set.
    fn extract_fonts_from_json(json: &Value) -> HashSet<String> {
        let mut fonts = HashSet::new();

        if let Some(family_map) = json.get("family_map").and_then(|v| v.as_object()) {
            for (_, fonts_list) in family_map {
                if let Some(array) = fonts_list.as_array() {
                    for item in array {
                        if let Some(font) = item.get("family").and_then(|v| v.as_str()) {
                            fonts.insert(font.to_string());
                        }
                    }
                }
            }
        }

        fonts
    }

    /// Replaces the `font_family` setting in the Kitty configuration file.
    ///
    /// This function updates the `font_family` setting in the Kitty configuration file specified by the `Config` struct.
    /// It uses a cached list of installed fonts to ensure that the specified font is valid and supported by the system.
    /// If the replacement is successful, it reloads the Kitty terminal to apply the new font family.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `Config` struct, which contains the path to the Kitty configuration file.
    /// * `new_font_family_no_spaces` - A string slice representing the new font family to set (without spaces).
    ///
    /// # Returns
    ///
    /// This function returns an `OxiResult<()>`. On success, the font family is updated, and Kitty is reloaded.
    /// On failure, an error (e.g., if the font is not found) is returned.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The specified font family is not found in the cached list of installed fonts.
    /// - The Kitty configuration file cannot be read or written due to I/O errors.
    /// - The Kitty reload command fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let config = Config {
    ///     kitty_conf_path: "~/.config/kitty/kitty.conf".to_string(),
    /// };
    ///
    /// Utils::replace_font_family(&config, "FiraCode")
    ///     .expect("Failed to replace font family");
    /// ```
    ///
    /// # Notes
    ///
    /// - This function relies on a cached list of installed fonts. Ensure that the font exists and is properly installed.
    /// - After modifying the configuration, Kitty is reloaded using a system command to apply the changes.
    pub fn replace_font_family(config: &Config, new_font_family_no_spaces: &str) -> OxiResult<()> {
        let config_path = Self::expand_tilde(&config.kitty_conf_path);
        let fonts_cache = Self::get_cached_installed_fonts();
        let formatted_font_name = fonts_cache.get(new_font_family_no_spaces).cloned();

        if let Some(new_font_family) = formatted_font_name {
            Self::modify_file_content(&config_path, |content| {
                Self::apply_regex_replace(&content, r"(?m)^(font_family\s+)(.*)", |caps| {
                    format!("{}{}", &caps[1], new_font_family)
                })
            })?;
            Self::reload_kitty();
            Ok(())
        } else {
            Err(PluginError::Custom("Font not found".to_string()).into())
        }
    }

    /// Replaces the `font_size` setting in the Kitty configuration file.
    ///
    /// This function updates the `font_size` setting in the Kitty configuration file specified by the `Config` struct.
    /// It modifies the font size to the provided `size` value and reloads the Kitty terminal to apply the changes.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `Config` struct, which contains the path to the Kitty configuration file.
    /// * `size` - A floating-point number representing the new font size to set.
    ///
    /// # Returns
    ///
    /// This function returns an `OxiResult<()>`. On success, the font size is updated, and Kitty is reloaded.
    /// On failure, an error (e.g., if the configuration file cannot be modified) is returned.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The Kitty configuration file cannot be read or written due to I/O errors.
    /// - The Kitty reload command fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let config = Config {
    ///     kitty_conf_path: "~/.config/kitty/kitty.conf".to_string(),
    /// };
    ///
    /// Utils::replace_font_size(&config, 14.0)
    ///     .expect("Failed to replace font size");
    /// ```
    ///
    /// # Notes
    ///
    /// - After modifying the configuration, Kitty is reloaded using a system command to apply the changes.
    pub fn replace_font_size(config: &Config, size: f32) -> OxiResult<()> {
        let config_path = Self::expand_tilde(&config.kitty_conf_path);

        Self::modify_file_content(&config_path, |content| {
            Self::apply_regex_replace(&content, r"(?m)^(font_size\s+)(.*)", |caps| {
                format!("{}{}", &caps[1], size)
            })
        })?;

        Self::reload_kitty();
        Ok(())
    }

    /// Returns a cached list of installed fonts with formatted names.
    ///
    /// This function retrieves the cached list of installed fonts, where the keys are font names without spaces,
    /// and the values are the original font names. If the cache is empty, it will populate the cache by calling
    /// `Utils::list_installed_fonts()` and formatting the font names by removing spaces. Subsequent calls will return
    /// the cached result, improving performance by avoiding repeated calls to list fonts.
    ///
    /// # Returns
    ///
    /// This function returns a reference to a static `HashMap<String, String>`, where:
    /// - Keys are font names formatted without spaces.
    /// - Values are the original font names.
    ///
    /// # Example
    ///
    /// ```rust
    /// let cached_fonts = Utils::get_cached_installed_fonts();
    /// for (formatted_name, original_name) in cached_fonts {
    ///     println!("Formatted: {}, Original: {}", formatted_name, original_name);
    /// }
    /// ```
    ///
    /// # Notes
    ///
    /// This function uses a `OnceLock` to ensure that the font list is only generated once and then cached.
    /// It is thread-safe and guarantees that the list will be initialized only once, even in a multi-threaded context.
    pub fn get_cached_installed_fonts() -> &'static HashMap<String, String> {
        FONT_CACHE.get_or_init(|| {
            let installed_fonts = Utils::list_installed_fonts();
            installed_fonts
                .into_iter()
                .map(|font| {
                    let formatted_font = font.replace(" ", "");
                    (formatted_font, font)
                })
                .collect()
        })
    }

    /// Reloads the Kitty terminal by sending the `USR1` signal to its process.
    ///
    /// This function finds the process ID (PID) of the running Kitty terminal using the `pidof` command and sends
    /// the `USR1` signal to the process using the `kill` command. This signal instructs Kitty to reload its configuration,
    /// applying any changes to the configuration file without needing to restart the terminal.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The `pidof` command fails to execute.
    /// - The `kill` command fails to execute.
    ///
    /// # Example
    ///
    /// ```rust
    /// // After modifying Kitty configuration, reload the terminal:
    /// Utils::reload_kitty();
    /// ```
    ///
    /// # Notes
    ///
    /// - This function only works on Unix-like systems where the `pidof` and `kill` commands are available.
    /// - If no Kitty process is found, the function silently does nothing.
    fn reload_kitty() {
        let pidof_output = Command::new("pidof")
            .arg("kitty")
            .output()
            .expect("Failed to execute `pidof`");

        if !pidof_output.stdout.is_empty() {
            let pid_list = String::from_utf8_lossy(&pidof_output.stdout);

            let pid = pid_list.trim();
            let _ = Command::new("sh")
                .arg("-c")
                .arg(format!("kill -USR1 {}", pid))
                .output()
                .expect("Failed to execute `kill`");
        }
    }

    /// Expands the tilde (`~`) in a file path to the user's home directory.
    ///
    /// This function checks if the provided path starts with a tilde (`~`), which is commonly used as a shorthand
    /// for the user's home directory on Unix-like systems. If the tilde is found, it is replaced with the absolute
    /// path to the home directory. If the home directory cannot be determined, it defaults to `/`.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice representing the file path that may contain a tilde (`~`).
    ///
    /// # Returns
    ///
    /// This function returns a `String` where the tilde, if present, is replaced with the user's home directory.
    /// If the path does not contain a tilde, the original path is returned unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// let expanded_path = Utils::expand_tilde("~/my_project/config");
    /// println!("Expanded path: {}", expanded_path);
    /// ```
    ///
    /// # Notes
    ///
    /// - This function uses the `dirs::home_dir()` function to retrieve the home directory path.
    /// - If the home directory cannot be determined, it defaults to `/` as a fallback.
    fn expand_tilde(path: &str) -> String {
        if path.starts_with("~") {
            let home_dir = home_dir().unwrap_or_else(|| PathBuf::from("/"));
            return path.replacen("~", &home_dir.to_string_lossy(), 1);
        }
        path.to_string()
    }
}
