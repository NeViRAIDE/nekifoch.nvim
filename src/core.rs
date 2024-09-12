use nvim_oxi::{
    api::{
        create_buf, err_writeln, open_win,
        opts::{OptionOpts, OptionScope},
        out_write,
        types::*,
        Buffer, Window,
    },
    Dictionary, Result as OxiResult, String as NvimString,
};

use crate::{error::PluginError, setup::Config};

use commands::{get_current_font, get_fonts_list, set_font_family, set_font_size};

use self::mapping::set_keymaps_for_buffer;

mod commands;
mod mapping;
mod window;

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

    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        let config = Config::from_dict(dict);
        self.config = config;
        Ok(())
    }

    fn set_buffer_content(buf: &mut Buffer, content: &str) -> OxiResult<()> {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        buf.set_lines(0.., true, lines)?;

        Ok(())
    }

    /// Получение центра окна на основе размеров редактора
    fn get_centered_window_position(
        &self,
        win_height: usize,
        win_width: usize,
    ) -> Result<(usize, usize), PluginError> {
        let opts = OptionOpts::default();

        // Получаем размеры редактора
        let editor_height: usize = nvim_oxi::api::get_option_value::<usize>("lines", &opts)
            .map_err(|e| PluginError::Custom(format!("Error getting editor height: {e}")))?;
        let editor_width: usize = nvim_oxi::api::get_option_value::<usize>("columns", &opts)
            .map_err(|e| PluginError::Custom(format!("Error getting editor width: {e}")))?;

        // Рассчитываем координаты для открытия окна по центру
        let row = (editor_height - win_height) / 2;
        let col = (editor_width - win_width) / 2;

        Ok((row, col))
    }

    pub fn open_window(&mut self, title: &str, items: Vec<String>) -> OxiResult<()> {
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

        let mut buf = create_buf(false, true)?;

        if let Err(err) = set_keymaps_for_buffer(&mut buf) {
            out_write(NvimString::from(format!(
                "Error setting buffer keymap: {}",
                err
            )));
        }

        let max_width = items.iter().map(|s| s.len()).max().unwrap_or(30);

        let content = items.join("\n");

        if let Err(err) = Self::set_buffer_content(&mut buf, &content) {
            out_write(NvimString::from(format!(
                "Error setting buffer content: {}",
                err
            )));
        }

        // Отключаем номера строк в этом окне
        // let buf_opts = OptionOpts::builder()
        //     .scope(OptionScope::Local)
        //     .win(self.window.clone().expect("there is no window"))
        //     .build();
        // nvim_oxi::api::set_option_value("number", false, &buf_opts)?;
        // nvim_oxi::api::set_option_value("relativenumber", false, &buf_opts)?;
        // if let Some(window) = &self.window {
        //     let buf_opts = OptionOpts::builder()
        //         .scope(OptionScope::Local) // Указываем локальную область применения
        //         .win(window.clone()) // Применяем опции к конкретному окну
        //         .build();
        //
        //     nvim_oxi::api::set_option_value("number", false, &buf_opts)?;
        //     nvim_oxi::api::set_option_value("relativenumber", false, &buf_opts)?;
        // } else {
        //     err_writeln("Window is not open.");
        // }

        // Рассчитываем размеры окна (например, 10 строк в высоту и 30 колонок в ширину)
        let win_height = 10;
        // let win_width = 30;
        // let win_height = items.len();
        let win_width = max_width + 4; // Добавляем небольшой отступ по ширине

        // Получаем центрированные координаты
        let (row, col) = self
            .get_centered_window_position(win_height, win_width)
            .map_err(|e| nvim_oxi::api::err_writeln(&format!("Error centering window: {}", e)))
            .unwrap_or((0, 0)); // В случае ошибки окно откроется в верхнем левом углу

        // Конфигурация окна
        let win_config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .row(row as f64)
            .col(col as f64)
            .height(win_height as u32)
            .width(win_width as u32)
            .title(WindowTitle::SimpleString(title.into()))
            .title_pos(WindowTitlePosition::Center)
            .border(win_border)
            .build();

        // Открываем окно и сохраняем ссылку на него
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

    pub fn handle_command(&mut self, cmd: &str, arg: Option<&str>) -> OxiResult<()> {
        match cmd {
            "close" => self.close_window(),
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
