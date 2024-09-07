use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

use nvim_oxi::{Dictionary, Result as OxiResult, String as NvimString};
use regex::Regex;
use serde_json::Value;

use crate::{error::ConfigError, Config};

pub struct Utils;

// #[derive(Debug, serde::Deserialize)]
// struct FontData {
//     #[serde(rename = "family")]
//     font_family: String,
// }

impl Utils {
    pub fn get(config: &Config) -> OxiResult<Dictionary> {
        let config_path = expand_tilde(&config.kitty_conf_path);
        if !Path::new(&config_path).exists() {
            return Err(ConfigError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", config_path),
            ))
            .into());
        }

        let file = File::open(config_path).map_err(ConfigError::from)?;
        let mut content = String::new();
        BufReader::new(file)
            .read_to_string(&mut content)
            .map_err(ConfigError::from)?;

        let mut current_font_family = None;
        let mut current_font_size = None;

        for line in content.lines() {
            if line.starts_with("font_family ") {
                current_font_family = Some(NvimString::from(
                    line.trim_start_matches("font_family ").trim(),
                ));
            } else if line.starts_with("font_size ") {
                current_font_size = Some(NvimString::from(
                    line.trim_start_matches("font_size ").trim(),
                ));
            }
        }

        Ok(Dictionary::from_iter(vec![
            ("font", current_font_family.unwrap_or_default()),
            ("size", current_font_size.unwrap_or_default()),
        ]))
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
        let config_path = expand_tilde(&config.kitty_conf_path);
        let mut content = String::new();
        {
            let file = File::open(&config_path)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error opening file: {e}"));
                    e
                })
                .map_err(ConfigError::from)?;
            BufReader::new(file)
                .read_to_string(&mut content)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error reading file: {e}"));
                    e
                })
                .map_err(ConfigError::from)?;
        }

        let mut cached_fonts = None;
        let fonts_cache = Utils::get_cached_installed_fonts(&mut cached_fonts);

        let formatted_font_name = fonts_cache.get(new_font_family_no_spaces).cloned();

        if let Some(new_font_family) = formatted_font_name {
            let font_re = Regex::new(r"(?m)^font_family .*").unwrap();
            let modified_content =
                font_re.replace_all(&content, format!("font_family {}", new_font_family));

            let mut file = BufWriter::new(
                File::create(config_path)
                    .map_err(|e| {
                        nvim_oxi::api::err_writeln(&format!("Error creating file: {e}"));
                        e
                    })
                    .map_err(ConfigError::from)?,
            );
            file.write_all(modified_content.as_bytes())
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error writing to file: {e}"));
                    e
                })
                .map_err(ConfigError::from)?;
            Ok(())
        } else {
            Err(ConfigError::Custom("Font not found".to_string()).into())
        }
    }

    pub fn replace_font_size(config: &Config, size: u32) -> OxiResult<()> {
        let config_path = expand_tilde(&config.kitty_conf_path);
        let mut content = String::new();
        {
            let file = File::open(&config_path)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error opening file: {e}"));
                    e
                })
                .map_err(ConfigError::from)?;
            BufReader::new(file)
                .read_to_string(&mut content)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error reading file: {e}"));
                    e
                })
                .map_err(ConfigError::from)?;
        }

        let size_re = Regex::new(r"(?m)^font_size .*").unwrap();
        let modified_content = size_re.replace_all(&content, format!("font_size {}", size));

        let mut file = BufWriter::new(
            File::create(config_path)
                .map_err(|e| {
                    nvim_oxi::api::err_writeln(&format!("Error creating file: {e}"));
                    e
                })
                .map_err(ConfigError::from)?,
        );
        file.write_all(modified_content.as_bytes())
            .map_err(|e| {
                nvim_oxi::api::err_writeln(&format!("Error writing to file: {e}"));
                e
            })
            .map_err(ConfigError::from)?;
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
                    let formatted_font = font.replace(" ", ""); // Убираем пробелы для ключа
                    (formatted_font, font) // Ключ — отформатированное имя, значение — оригинальное
                })
                .collect();
            *cached_fonts = Some(formatted_fonts);
        }
        cached_fonts.as_ref().unwrap()
    }
}

// fn extract_fonts(json_str: &str) -> HashSet<String> {
//     let mut fonts = HashSet::new();
//     for family in json_str.split('"').filter(|&s| s.contains("family")) {
//         if let Some(font) = family.split(':').nth(1) {
//             fonts.insert(font.trim().replace('"', "").to_string());
//         }
//     }
//     fonts
// }

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~") {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        return path.replacen("~", &home_dir.to_string_lossy(), 1);
    }
    path.to_string()
}
