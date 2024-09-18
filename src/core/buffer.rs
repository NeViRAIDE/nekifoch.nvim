use nvim_oxi::{
    api::{
        opts::{OptionOpts, OptionScope},
        set_option_value, Buffer, Window,
    },
    Result as OxiResult,
};

pub struct BufferManager;

impl BufferManager {
    pub fn set_buffer_content(buf: &mut Buffer, content: &str) -> OxiResult<()> {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        buf.set_lines(0.., true, lines)?;
        Ok(())
    }

    // TODO: set buffer type
    pub fn configure_buffer(window: &Window) -> OxiResult<()> {
        let buf_opts = OptionOpts::builder()
            .scope(OptionScope::Local)
            .win(window.clone())
            .build();

        set_option_value("number", false, &buf_opts)?;
        set_option_value("relativenumber", false, &buf_opts)?;
        // set_option_value("cursorline", false, &buf_opts)?;
        Ok(())
    }
}
