use nvim_oxi::{
    api::{opts::SetKeymapOpts, types::Mode, Buffer},
    Result as OxiResult,
};

pub const CLOSE_COMMAND: &str = "<cmd>Nekifoch close<cr>";
pub const RETURN_COMMAND: &str = r#"<cmd>lua vim.cmd('Nekifoch close'); vim.cmd('Nekifoch')<cr>"#;

pub fn set_keymaps_for_buffer(
    buf: &mut Buffer,
    enter_cmd: &str,
    close_cmd: &str,
    back_cmd: &str,
) -> OxiResult<()> {
    let opts = SetKeymapOpts::builder().noremap(true).silent(true).build();

    // Set the keymap for the Enter key using the provided `enter_cmd`.
    buf.set_keymap(Mode::Normal, "<CR>", enter_cmd, &opts)?;

    // Set the keymap for the 'q' key or 'Esc' key using the provided `close_cmd`.
    buf.set_keymap(Mode::Normal, "q", close_cmd, &opts)?;
    buf.set_keymap(Mode::Normal, "<Esc>", back_cmd, &opts)?;

    // Optional navigation keymaps, if needed.
    buf.set_keymap(Mode::Normal, "j", "gj", &opts)?;
    buf.set_keymap(Mode::Normal, "<Down>", "gj", &opts)?;
    buf.set_keymap(Mode::Normal, "<Tab>", "gj", &opts)?;
    buf.set_keymap(Mode::Normal, "k", "gk", &opts)?;
    buf.set_keymap(Mode::Normal, "<Up>", "gk", &opts)?;
    buf.set_keymap(Mode::Normal, "<S-Tab>", "gk", &opts)?;

    Ok(())
}

pub fn set_keymaps_for_menu(buf: &mut Buffer) -> OxiResult<()> {
    let opts = SetKeymapOpts::builder().noremap(true).silent(true).build();

    buf.set_keymap(
        Mode::Normal,
        "<CR>",
        r#"<cmd>lua local selection = vim.api.nvim_get_current_line(); if selection == "Check current font" then vim.cmd('Nekifoch check') elseif selection == "Show installed fonts" then vim.cmd('Nekifoch list') elseif selection == "Set font family" then vim.cmd('Nekifoch close');vim.cmd('Nekifoch set_font') elseif selection == "Set font size" then vim.cmd('Nekifoch close');vim.cmd('Nekifoch set_size') end<CR>"#,
        &opts,
    )?;

    buf.set_keymap(Mode::Normal, "q", CLOSE_COMMAND, &opts)?;
    buf.set_keymap(Mode::Normal, "<Esc>", CLOSE_COMMAND, &opts)?;

    Ok(())
}
