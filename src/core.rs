use nvim_oxi::{
    api::{create_buf, err_writeln, open_win, types::*, Window},
    print, Dictionary, Result as OxiResult,
};

use crate::{setup::Config, utils::Utils};

#[derive(Debug)]
pub struct App {
    config: Config,
    window: Option<Window>,
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            config,
            window: None,
        }
    }

    pub fn open_window(&mut self) -> OxiResult<()> {
        if self.window.is_some() {
            err_writeln("Window is already open");
            return Ok(());
        }

        let win_border = match self.config.border.as_str() {
            "double" => WindowBorder::Double,
            "single" => WindowBorder::Single,
            "rounded" => WindowBorder::Rounded,
            "solid" => WindowBorder::Solid,
            "shadow" => WindowBorder::Shadow,
            _ => WindowBorder::None,
        };

        let buf = create_buf(false, true)?;
        let win_config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .row(50)
            .col(50)
            .height(10)
            .width(15)
            .title(WindowTitle::SimpleString("Nekifoch".into()))
            .title_pos(WindowTitlePosition::Center)
            .border(win_border)
            .build();

        self.window = Some(open_win(&buf, true, &win_config)?);

        Ok(())
    }

    pub fn close_window(&mut self) -> OxiResult<()> {
        if self.window.is_none() {
            err_writeln("Window is already closed");
            return Ok(());
        }

        if let Some(win) = self.window.take() {
            win.close(false).map_err(|e| e.into())
        } else {
            Ok(())
        }
    }

    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        let config = Config::from_dict(dict);
        self.config = config;
        Ok(())
    }

    pub fn handle_command(&mut self, cmd: &str, arg: Option<&str>) -> OxiResult<()> {
        match cmd {
            "open" => self.open_window(),
            "close" => self.close_window(),
            "check" => {
                let fonts = Utils::get(&self.config)?;
                print!(
                    "Font family: {:?}\nFont size: {:?}",
                    fonts["font"], fonts["size"]
                );
                Ok(())
            }
            "set_font" => {
                if let Some(font_family) = arg {
                    Utils::replace_font_family(&self.config, font_family)?;
                    nvim_oxi::api::out_write(nvim_oxi::String::from(format!(
                        "Font family set to {}",
                        font_family
                    )));
                } else {
                    err_writeln("Missing font family argument for set_font action");
                }
                Ok(())
            }
            "set_size" => {
                if let Some(size_str) = arg {
                    if let Ok(size) = size_str.parse::<f32>() {
                        Utils::replace_font_size(&self.config, size)?;
                        nvim_oxi::api::out_write(nvim_oxi::String::from(format!(
                            "Font size set to {}",
                            size
                        )));
                    } else {
                        err_writeln("Invalid font size argument for set_size action");
                    }
                } else {
                    err_writeln("Missing font size argument for set_size action");
                }
                Ok(())
            }
            "list" => {
                let installed_fonts = Utils::list_installed_fonts();
                let compatible = Utils::compare_fonts_with_kitty_list_fonts(installed_fonts);
                print!("Available fonts:");
                for font in compatible.values() {
                    print!("  - {font}");
                }
                Ok(())
            }
            _ => {
                err_writeln(&format!("Unknown command: {}", cmd));
                Ok(())
            }
        }
    }
}
