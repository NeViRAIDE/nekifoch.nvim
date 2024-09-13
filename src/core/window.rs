use nvim_oxi::{
    api::{
        create_buf, err_writeln, get_option_value, open_win, opts::OptionOpts, out_write, types::*,
        Window,
    },
    Result as OxiResult, String as NvimString,
};

use crate::{error::PluginError, setup::Config};

use super::{buffer::BufferManager, mapping::set_keymaps_for_buffer};

#[derive(Debug)]
pub struct FloatWindow {
    pub window: Option<Window>,
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

    pub fn open(&mut self, config: &Config, title: &str, items: Vec<String>) -> OxiResult<()> {
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

        if let Err(err) = set_keymaps_for_buffer(&mut buf) {
            out_write(NvimString::from(format!(
                "Error setting buffer keymap: {}",
                err
            )));
        }

        let max_width = items.iter().map(|s| s.len()).max().unwrap_or(30);

        let content = items.join("\n");

        if let Err(err) = BufferManager::set_buffer_content(&mut buf, &content) {
            out_write(NvimString::from(format!(
                "Error setting buffer content: {}",
                err
            )));
        }

        let win_height = 10;
        let win_width = max_width + 4;

        let (row, col) = self
            .get_centered_position(win_height, win_width)
            .map_err(|e| err_writeln(&format!("Error centering window: {}", e)))
            .unwrap_or((0, 0));

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

        self.window = Some(open_win(&buf, true, &win_config)?);

        Ok(())
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
