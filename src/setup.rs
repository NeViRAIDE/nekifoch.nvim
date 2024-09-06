use nvim_oxi::{conversion::FromObject, Dictionary};

#[derive(Debug, Default)]
pub struct Config {
    pub border: String,
    pub kitty_conf_path: String,
    pub which_key_enable: bool,
}

impl Config {
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

            which_key_enable: options
                .get("which_key")
                .and_then(|which_key_obj| Dictionary::from_object(which_key_obj.clone()).ok())
                .and_then(|which_key_dict| {
                    which_key_dict
                        .get("enable")
                        .and_then(|enable_obj| bool::from_object(enable_obj.clone()).ok())
                })
                .unwrap_or(false),
        }
    }
}
