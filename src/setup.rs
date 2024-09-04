use nvim_oxi::{conversion::FromObject, print, Dictionary};

#[derive(Debug, Default)]
pub struct Config {
    pub border: String,
    pub kitty_conf_path: String,
    pub which_key_enable: bool,
}

impl Config {
    pub fn from_dict(options: Dictionary) -> Self {
        let mut config = Config::default();

        if let Some(border_obj) = options.get("borders") {
            if let Ok(border_str) = String::from_object(border_obj.clone()) {
                print!("Using custom border: {}", border_str);
                config.border = border_str;
            } else {
                print!("Border option is not a valid string");
            }
        }

        if let Some(path_obj) = options.get("kitty_conf_path") {
            if let Ok(path_str) = String::from_object(path_obj.clone()) {
                print!("Using custom kitty_conf_path: {}", path_str);
                config.kitty_conf_path = path_str;
            } else {
                print!("kitty_conf_path option is not a valid string");
            }
        }

        if let Some(which_key_obj) = options.get("which_key") {
            if let Ok(which_key_dict) = Dictionary::from_object(which_key_obj.clone()) {
                if let Some(enable_obj) = which_key_dict.get("enable") {
                    if let Ok(enable_bool) = bool::from_object(enable_obj.clone()) {
                        print!("Which_key enabled: {}", enable_bool);
                        config.which_key_enable = enable_bool;
                    } else {
                        print!("which_key.enable option is not a valid boolean");
                    }
                }
            } else {
                print!("which_key option is not a valid dictionary");
            }
        }

        config
    }
}
