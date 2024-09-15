use nvim_oxi::{
    api::{
        create_buf, err_writeln, get_option_value, open_win, opts::OptionOpts, types::*, Window,
    },
    Result as OxiResult,
};

use crate::{error::PluginError, setup::Config};

use super::{
    buffer::BufferManager,
    mapping::{set_keymaps_for_buffer, CLOSE_COMMAND},
};

#[derive(Debug)]
pub struct FloatWindow {
    pub window: Option<Window>,
}

pub struct WindowConfigParams<'a> {
    pub title: &'a str,
    pub height: usize,
    pub width: usize,
    pub insert_mode: bool,
    pub set_keymaps: bool,
    pub content: Option<&'a str>,
    pub enter_cmd: Option<&'a str>,
    pub close_cmd: Option<&'a str>,
}

impl<'a> WindowConfigParams<'a> {
    pub fn new(title: &'a str, height: usize, width: usize) -> Self {
        Self {
            title,
            height,
            width,
            insert_mode: false,
            set_keymaps: false,
            content: None,
            enter_cmd: None,
            close_cmd: Some(CLOSE_COMMAND),
        }
    }

    pub fn with_insert_mode(mut self, insert: bool) -> Self {
        self.insert_mode = insert;
        self
    }

    pub fn with_keymaps(mut self, keymaps: bool) -> Self {
        self.set_keymaps = keymaps;
        self
    }

    pub fn with_content(mut self, content: Option<&'a str>) -> Self {
        self.content = content;
        self
    }

    pub fn with_enter_cmd(mut self, cmd: Option<&'a str>) -> Self {
        self.enter_cmd = cmd;
        self
    }
}

impl FloatWindow {
    pub fn new() -> Self {
        Self { window: None }
    }

    fn get_centered_position(
        &self,
        win_height: usize,
        win_width: usize,
    ) -> Result<(usize, usize), PluginError> {
        let opts = OptionOpts::default();

        let editor_height: usize = get_option_value::<usize>("lines", &opts)
            .map_err(|e| PluginError::Custom(format!("Error getting editor height: {e}")))?;
        let editor_width: usize = get_option_value::<usize>("columns", &opts)
            .map_err(|e| PluginError::Custom(format!("Error getting editor width: {e}")))?;

        let row = (editor_height - win_height) / 2;
        let col = (editor_width - win_width) / 2;

        Ok((row, col))
    }

    fn open_window(&mut self, config: &Config, params: WindowConfigParams) -> OxiResult<()> {
        if self.window.is_some() {
            err_writeln("Window is already open");
            return Ok(());
        }

        let win_border = match config.border.as_str() {
            "double" => WindowBorder::Double,
            "single" => WindowBorder::Single,
            "rounded" => WindowBorder::Rounded,
            "solid" => WindowBorder::Solid,
            "shadow" => WindowBorder::Shadow,
            _ => WindowBorder::None,
        };

        let mut buf = create_buf(false, true)?;

        if let Some(content) = params.content {
            BufferManager::set_buffer_content(&mut buf, content)?;
        }

        if params.set_keymaps {
            if let Some(enter_cmd) = params.enter_cmd {
                if let Some(close_cmd) = params.close_cmd {
                    set_keymaps_for_buffer(&mut buf, enter_cmd, close_cmd)?;
                }
            }
        }

        let (row, col) = self
            .get_centered_position(params.height, params.width)
            .map_err(|e| err_writeln(&format!("Error centering window: {}", e)))
            .unwrap_or((0, 0));

        let win_config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .row(row as f64)
            .col(col as f64)
            .height(params.height as u32)
            .width(params.width as u32)
            .title(WindowTitle::SimpleString(params.title.into()))
            .title_pos(WindowTitlePosition::Center)
            .border(win_border)
            .build();

        self.window = Some(open_win(&buf, true, &win_config)?);

        if params.insert_mode {
            nvim_oxi::api::command("startinsert")?;
        }

        Ok(())
    }

    pub fn open(&mut self, config: &Config, title: &str, items: Vec<String>) -> OxiResult<()> {
        let content = items.join("\n");
        let max_width = items.iter().map(|s| s.len()).max().unwrap_or(30);

        let params = WindowConfigParams::new(title, 10, max_width + 4)
        .with_content(Some(&content))
        .with_keymaps(true)
        .with_enter_cmd(Some(r#"<cmd>lua local font_name = vim.api.nvim_get_current_line(); local formatted_font_name = font_name:gsub('%s+', ''); vim.cmd('Nekifoch set_font ' .. formatted_font_name)<CR>"#));

        self.open_window(config, params)
    }

    pub fn open_for_input(&mut self, config: &Config, title: &str) -> OxiResult<()> {
        let params = WindowConfigParams::new(title, 2, 20)
        .with_insert_mode(true)
        .with_keymaps(true)
        .with_enter_cmd(Some(r#"<cmd>lua local font_size = vim.api.nvim_get_current_line(); vim.cmd('Nekifoch set_size ' .. font_size)<CR>"#));

        self.open_window(config, params)
    }

    pub fn close(&mut self) -> OxiResult<()> {
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
}
