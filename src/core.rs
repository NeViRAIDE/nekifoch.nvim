use nvim_oxi::{api::err_writeln, Dictionary, Result as OxiResult};

use crate::setup::Config;

use commands::{get_current_font, get_fonts_list, set_font_family, set_font_size};
use window::FloatWindow;

mod buffer;
mod commands;
mod mapping;
mod window;
pub mod completion;

#[derive(Debug)]
pub struct App {
    config: Config,
    float_window: FloatWindow,
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            config,
            float_window: FloatWindow::new(),
        }
    }

    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        let config = Config::from_dict(dict);
        self.config = config;
        Ok(())
    }

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
