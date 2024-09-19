use nvim_oxi::{
    api::{
        clear_autocmds, create_autocmd, err_writeln,
        opts::{ClearAutocmdsOpts, CreateAutocmdOpts, OptionOpts, OptionScope},
        set_option_value, Buffer,
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

    pub fn configure_buffer() -> OxiResult<()> {
        let buf_opts = OptionOpts::builder().scope(OptionScope::Local).build();

        if let Err(e) = set_option_value("number", false, &buf_opts) {
            err_writeln(&format!("Failed to set 'number' option: {}", e));
        }

        if let Err(e) = set_option_value("relativenumber", false, &buf_opts) {
            err_writeln(&format!("Failed to set 'relativenumber' option: {}", e));
        }

        if let Err(e) = set_option_value("filetype", "nekifoch", &buf_opts) {
            err_writeln(&format!("Failed to set 'filetype': {}", e));
        }

        Ok(())
    }

    pub fn setup_autocmd_for_float_window(buffer: &Buffer) -> OxiResult<()> {
        let clear_opts = ClearAutocmdsOpts::builder().buffer(buffer.clone()).build();
        clear_autocmds(&clear_opts)?;

        let autocmd_opts = CreateAutocmdOpts::builder()
            .command("Nekifoch close")
            .buffer(buffer.clone())
            .build();

        create_autocmd(vec!["WinLeave"], &autocmd_opts)?;

        Ok(())
    }
}
