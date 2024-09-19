use nvim_oxi::{
    api::{
        create_buf, create_namespace, err_writeln, open_win, opts::OptionOpts, set_option_value,
        types::*, Window,
    },
    Result as OxiResult,
};

use crate::{setup::Config, utils::Utils};

use super::{
    buffer::BufferManager,
    mapping::{
        set_common_keymaps, set_keymaps_for_family_control, set_keymaps_for_size_control,
        set_menu_keymaps, SIZE_DOWN_COMMAND, SIZE_UP_COMMAND,
    },
};

#[derive(Debug)]
pub struct FloatWindow {
    pub window: Option<Window>,
}

pub enum WindowType {
    FontSizeControl,
    FontFamilyMenu,
    MainMenu,
    FontInfo,
    FontList,
}

pub struct CustomWindowConfig<'a> {
    title: &'a str,
    height: usize,
    width: usize,
    content: Option<&'a str>,
    keymaps: bool,
    window_type: WindowType,
}

impl<'a> CustomWindowConfig<'a> {
    pub fn new(title: &'a str, height: usize, width: usize, window_type: WindowType) -> Self {
        Self {
            title,
            height,
            width,
            content: None,
            keymaps: false,
            window_type,
        }
    }

    pub fn with_keymaps(mut self, keymaps: bool) -> Self {
        self.keymaps = keymaps;
        self
    }

    pub fn with_content(mut self, content: Option<&'a str>) -> Self {
        self.content = content;
        self
    }
}

impl FloatWindow {
    pub fn new() -> Self {
        Self { window: None }
    }

    fn create_window(
        &mut self,
        config: &Config,
        window_config: CustomWindowConfig,
    ) -> OxiResult<()> {
        self.open_window(config, &window_config)?;

        if let Some(window) = &self.window {
            let buf = window.get_buf()?;
            BufferManager::configure_buffer()?;
            BufferManager::setup_autocmd_for_float_window(&buf)?;
        }

        Ok(())
    }

    fn open_window(&mut self, config: &Config, params: &CustomWindowConfig) -> OxiResult<()> {
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

        set_common_keymaps(&mut buf)?;

        if params.keymaps {
            match params.window_type {
                WindowType::FontFamilyMenu => {
                    set_common_keymaps(&mut buf)?;
                    set_keymaps_for_family_control(&mut buf)?;
                }
                WindowType::FontSizeControl => {
                    set_common_keymaps(&mut buf)?;
                    set_keymaps_for_size_control(&mut buf, SIZE_UP_COMMAND, SIZE_DOWN_COMMAND)?;
                }
                WindowType::MainMenu => {
                    set_menu_keymaps(&mut buf)?;
                }
                WindowType::FontInfo => {
                    set_common_keymaps(&mut buf)?;
                }
                WindowType::FontList => {
                    set_common_keymaps(&mut buf)?;
                }
            }
        }

        let (row, col) = Utils
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
            // .footer(WindowTitle::SimpleString(" [ Back: <esc> | Up: <k> | Down: <j> | Quit: <q> ] ".into()))
            // .footer_pos(WindowTitlePosition::Right)
            .border(win_border)
            .build();

        self.window = Some(open_win(&buf, true, &win_config)?);

        Ok(())
    }

    pub fn f_family_win(
        &mut self,
        config: &Config,
        title: &str,
        items: Vec<String>,
        win_height: usize,
        current_font: &str, // добавляем текущий шрифт
    ) -> OxiResult<()> {
        let binding = items.join("\n");
        let content = Some(binding.as_str());

        let window_config = CustomWindowConfig::new(
            title,
            win_height,
            items.iter().map(|s| s.len()).max().unwrap_or(30) + 4,
            WindowType::FontFamilyMenu,
        )
        .with_content(content)
        .with_keymaps(true);

        self.create_window(config, window_config)?;

        if let Some(window) = &mut self.window {
            if let Some(index) = items.iter().position(|f| f == current_font) {
                window.set_cursor(index + 1, 0)?;
            }
        }

        Ok(())
    }

    pub fn f_size_win(&mut self, config: &Config, title: &str, current_size: f32) -> OxiResult<()> {
        let content = format!("\t\t\t\t\nCurrent size: [ {} ]\n\t\t\t\t", current_size);

        let window_config = CustomWindowConfig::new(title, 3, 25, WindowType::FontSizeControl)
            .with_content(Some(&content))
            .with_keymaps(true);

        self.create_window(config, window_config)?;

        if let Some(window) = self.window.as_mut() {
            let mut buf = window.get_buf()?;
            window.set_cursor(2, 16)?;

            let ns_id = create_namespace("font_size_namespace");
            buf.add_highlight(ns_id, "Comment", 1, 0..13)?;
            set_option_value(
                "cursorline",
                false,
                &OptionOpts::builder().win(window.clone()).build(),
            )?;
        }

        Ok(())
    }

    pub fn menu_win(&mut self, config: &Config) -> OxiResult<()> {
        let menu_options = [
            "Check current font".to_string(),
            "Set font family".to_string(),
            "Set font size".to_string(),
            "Show installed fonts".to_string(),
        ];

        let binding = menu_options.join("\n");
        let content = Some(binding.as_str());

        let window_config = CustomWindowConfig::new(
            " Nekifoch ",
            4,
            menu_options.iter().map(|s| s.len()).max().unwrap_or(30) + 4,
            WindowType::MainMenu,
        )
        .with_content(content)
        .with_keymaps(true);

        self.create_window(config, window_config)?;

        Ok(())
    }

    pub fn f_check_win(
        &mut self,
        config: &Config,
        title: &str,
        content: Option<&str>,
        win_height: usize,
    ) -> OxiResult<()> {
        let window_width = content
            .map(|s| s.lines().map(|line| line.len()).max().unwrap_or(20))
            .unwrap_or(20)
            + 4;

        let window_config =
            CustomWindowConfig::new(title, win_height, window_width, WindowType::FontInfo)
                .with_content(content)
                .with_keymaps(true);

        self.create_window(config, window_config)?;

        if let Some(window) = self.window.as_mut() {
            let mut buf = window.get_buf()?;

            BufferManager::configure_buffer()?;
            window.set_cursor(1, 8)?;

            let ns_id = create_namespace("font_size_namespace");
            buf.add_highlight(ns_id, "Comment", 0, 0..7)?;
            buf.add_highlight(ns_id, "Comment", 1, 0..7)?;

            set_option_value(
                "cursorline",
                false,
                &OptionOpts::builder().win(window.clone()).build(),
            )?;
        }

        Ok(())
    }

    pub fn f_list_win(
        &mut self,
        config: &Config,
        title: &str,
        content: String,
        win_height: usize,
    ) -> OxiResult<()> {
        let window_config = CustomWindowConfig::new(
            title,
            win_height,
            content.lines().map(|s| s.len()).max().unwrap_or(20) + 4,
            WindowType::FontList,
        )
        .with_content(Some(&content))
        .with_keymaps(true);

        self.create_window(config, window_config)?;

        if let Some(window) = &self.window {
            BufferManager::configure_buffer()?;

            set_option_value(
                "cursorline",
                false,
                &OptionOpts::builder().win(window.clone()).build(),
            )?;
        }

        Ok(())
    }

    pub fn close_win(&mut self) -> OxiResult<()> {
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
