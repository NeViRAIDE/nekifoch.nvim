use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use nvim_oxi::Result as OxiResult;
use regex::Regex;
use serde_json::Value;

use crate::{error::PluginError, Config};

pub struct Utils;

impl Utils {
    pub fn get(config: &Config) -> OxiResult<HashMap<String, String>> {
        let config_path = Self::expand_tilde(&config.kitty_conf_path);
        if !Path::new(&config_path).exists() {
            return Err(PluginError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", config_path),
            ))
            .into());
        }

        let file = File::open(config_path).map_err(PluginError::from)?;
        let mut content = String::new();
        BufReader::new(file)
            .read_to_string(&mut content)
            .map_err(PluginError::from)?;

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

    pub fn replace_font_family(config: &Config, new_font_family_no_spaces: &str) -> OxiResult<()> {
        let config_path = Self::expand_tilde(&config.kitty_conf_path);
        let mut content = String::new();
        {
            let file = File::open(&config_path)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error opening file: {e}"));
                    e
                })
                .map_err(PluginError::from)?;
            BufReader::new(file)
                .read_to_string(&mut content)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error reading file: {e}"));
                    e
                })
                .map_err(PluginError::from)?;
        }

        let mut cached_fonts = None;
        let fonts_cache = Self::get_cached_installed_fonts(&mut cached_fonts);

        let formatted_font_name = fonts_cache.get(new_font_family_no_spaces).cloned();

        if let Some(new_font_family) = formatted_font_name {
            let font_re = Regex::new(r"(?m)^(font_family\s+)(.*)").unwrap();
            let modified_content = font_re.replace_all(&content, |caps: &regex::Captures| {
                let indent = &caps[1];
                let _old_font = &caps[2];
                format!("{}{}", indent, new_font_family)
            });

            let mut file = BufWriter::new(
                File::create(config_path)
                    .map_err(|e| {
                        nvim_oxi::api::err_writeln(&format!("Error creating file: {e}"));
                        e
                    })
                    .map_err(PluginError::from)?,
            );
            file.write_all(modified_content.as_bytes())
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error writing to file: {e}"));
                    e
                })
                .map_err(PluginError::from)?;

            Self::reload_kitty();
            Ok(())
        } else {
            Err(PluginError::Custom("Font not found".to_string()).into())
        }
    }

    pub fn replace_font_size(config: &Config, size: f32) -> OxiResult<()> {
        let config_path = Self::expand_tilde(&config.kitty_conf_path);
        let mut content = String::new();
        {
            let file = File::open(&config_path)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error opening file: {e}"));
                    e
                })
                .map_err(PluginError::from)?;
            BufReader::new(file)
                .read_to_string(&mut content)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error reading file: {e}"));
                    e
                })
                .map_err(PluginError::from)?;
        }

        // TODO: separate as util (using twice)
        let size_re = Regex::new(r"(?m)^(font_size\s+)(.*)").unwrap();
        let modified_content = size_re.replace_all(&content, |caps: &regex::Captures| {
            let indent = &caps[1];
            let _old_size = &caps[2];
            format!("{}{}", indent, size)
        });

        let mut file = BufWriter::new(
            File::create(config_path)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error creating file: {e}"));
                    e
                })
                .map_err(PluginError::from)?,
        );
        file.write_all(modified_content.as_bytes())
            .map_err(|e| {
                nvim_oxi::api::err_writeln(&format!("Error writing to file: {e}"));
                e
            })
            .map_err(PluginError::from)?;

        Self::reload_kitty();
        Ok(())
    }

    pub fn get_cached_installed_fonts(
        cached_fonts: &mut Option<HashMap<String, String>>,
    ) -> &HashMap<String, String> {
        if cached_fonts.is_none() {
            let installed_fonts = Self::list_installed_fonts();
            let formatted_fonts: HashMap<String, String> = installed_fonts
                .into_iter()
                .map(|font| {
                    let formatted_font = font.replace(" ", "");
                    (formatted_font, font)
                })
                .collect();
            *cached_fonts = Some(formatted_fonts);
        }
        cached_fonts.as_ref().unwrap()
    }

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

    fn expand_tilde(path: &str) -> String {
        if path.starts_with("~") {
            let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
            return path.replacen("~", &home_dir.to_string_lossy(), 1);
        }
        path.to_string()
    }
}
