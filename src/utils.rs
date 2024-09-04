use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    process::Command,
};

use nvim_oxi::{Dictionary, Result as OxiResult, String as NvimString};

use crate::{error::ConfigError, CONFIG};

pub struct Utils;

impl Utils {
    pub fn get() -> OxiResult<Dictionary> {
        let config_path = CONFIG.with(|c| c.borrow().kitty_conf_path.clone());
        let file = File::open(&config_path).map_err(ConfigError::from)?;
        let mut content = String::new();
        BufReader::new(file)
            .read_to_string(&mut content)
            .map_err(ConfigError::from)?;

        let mut current_font_family = None;
        let mut current_font_size = None;

        for line in content.lines() {
            if line.starts_with("font_family ") {
                current_font_family = line.split_whitespace().nth(1).map(NvimString::from);
            } else if line.starts_with("font_size ") {
                current_font_size = line.split_whitespace().nth(1).map(NvimString::from);
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
    ) -> (Vec<String>, Vec<String>) {
        let cmd = r#"kitty +runpy "from kitty.fonts.common import all_fonts_map; import json; print(json.dumps(all_fonts_map(True), indent=2))" 2>/dev/null"#;
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        let result = String::from_utf8_lossy(&output.stdout);
        let kitty_fonts = extract_fonts(&result);
        let mut compatible_fonts = Vec::new();
        let mut compatible_formatted_fonts = Vec::new();

        for font in installed_fonts {
            if kitty_fonts.contains(&font) {
                compatible_fonts.push(font.clone());
            }
            let formatted_font = font.replace(" ", "");
            if kitty_fonts.contains(&formatted_font) {
                compatible_formatted_fonts.push(formatted_font);
            }
        }

        (compatible_formatted_fonts, compatible_fonts)
    }

    pub fn replace_font_family(new_font_family: &str) -> OxiResult<()> {
        let config_path = CONFIG.with(|c| c.borrow().kitty_conf_path.clone());
        let mut content = String::new();
        {
            let file = File::open(&config_path).map_err(ConfigError::from)?;
            BufReader::new(file)
                .read_to_string(&mut content)
                .map_err(ConfigError::from)?;
        }

        let modified_content =
            content.replace("font_family", &format!("font_family {}", new_font_family));

        let mut file = BufWriter::new(File::create(&config_path).map_err(ConfigError::from)?);
        file.write_all(modified_content.as_bytes())
            .map_err(ConfigError::from)?;

        Ok(())
    }

    pub fn replace_font_size(size: u32) -> OxiResult<()> {
        let config_path = CONFIG.with(|c| c.borrow().kitty_conf_path.clone());
        let mut content = String::new();
        {
            let file = File::open(&config_path).map_err(ConfigError::from)?;
            BufReader::new(file)
                .read_to_string(&mut content)
                .map_err(ConfigError::from)?;
        }

        let modified_content = content.replace("font_size", &format!("font_size {}", size));

        let mut file = BufWriter::new(File::create(&config_path).map_err(ConfigError::from)?);
        file.write_all(modified_content.as_bytes())
            .map_err(ConfigError::from)?;

        Ok(())
    }

    pub fn get_cached_installed_fonts(cached_fonts: &mut Option<Vec<String>>) -> &Vec<String> {
        if cached_fonts.is_none() {
            *cached_fonts = Some(Utils::list_installed_fonts());
        }
        cached_fonts.as_ref().unwrap()
    }
}

fn extract_fonts(json_str: &str) -> HashSet<String> {
    let mut fonts = HashSet::new();
    for family in json_str.split('"').filter(|&s| s.contains("family")) {
        if let Some(font) = family.split(':').nth(1) {
            fonts.insert(font.trim().replace('"', "").to_string());
        }
    }
    fonts
}
