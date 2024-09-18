use nvim_oxi::{
    api::{opts::SetKeymapOpts, types::Mode, Buffer},
    Result as OxiResult,
};

pub const CLOSE_COMMAND: &str = "<cmd>lua vim.cmd('Nekifoch close')<cr>";
pub const BACK_COMMAND: &str = r#"<cmd>lua vim.cmd('Nekifoch close'); vim.cmd('Nekifoch')<cr>"#;
pub const SIZE_UP_COMMAND: &str = "<cmd>Nekifoch size_up<CR>";
pub const SIZE_DOWN_COMMAND: &str = "<cmd>Nekifoch size_down<CR>";

pub fn set_keymaps(
    buf: &mut Buffer,
    mappings: Vec<(&str, &str)>,
    opts: SetKeymapOpts,
) -> OxiResult<()> {
    for (key, cmd) in mappings {
        buf.set_keymap(Mode::Normal, key, cmd, &opts)?;
    }
    Ok(())
}

pub fn set_common_keymaps(buf: &mut Buffer) -> OxiResult<()> {
    let mappings = vec![
        ("q", CLOSE_COMMAND),
        ("<Esc>", BACK_COMMAND),
        ("<BS>", BACK_COMMAND),
    ];

    set_keymaps(
        buf,
        mappings,
        SetKeymapOpts::builder().noremap(true).silent(true).build(),
    )
}

pub fn set_keymaps_for_size_control(
    buf: &mut Buffer,
    size_up_cmd: &str,
    size_down_cmd: &str,
) -> OxiResult<()> {
    let mappings = vec![
        ("k", size_up_cmd),
        ("j", size_down_cmd),
        ("<up>", size_up_cmd),
        ("<down>", size_down_cmd),
    ];

    set_keymaps(
        buf,
        mappings,
        SetKeymapOpts::builder().noremap(true).silent(true).build(),
    )
}

pub fn set_keymaps_for_family_control(buf: &mut Buffer) -> OxiResult<()> {
    let get_current_line = "local font_name = vim.api.nvim_get_current_line();";
    let format_font_name = "local formatted_font_name = font_name:gsub('%s+', '');";
    let set_font_cmd = "vim.cmd('Nekifoch set_font ' .. formatted_font_name);";

    let lua_cmd = format!(
        "<cmd>lua {}; {}; {}<CR>",
        get_current_line, format_font_name, set_font_cmd
    );

    let mappings = vec![("<CR>", lua_cmd.as_str())];

    set_keymaps(
        buf,
        mappings,
        SetKeymapOpts::builder().noremap(true).silent(true).build(),
    )
}

pub fn set_menu_keymaps(buf: &mut Buffer) -> OxiResult<()> {
    let check_font = r#"if selection == "Check current font" then vim.cmd('Nekifoch check')"#;
    let show_installed_fonts =
        r#"elseif selection == "Show installed fonts" then vim.cmd('Nekifoch list')"#;
    let set_font_family = r#"elseif selection == "Set font family" then vim.cmd('Nekifoch close'); vim.cmd('Nekifoch set_font')"#;
    let set_font_size = r#"elseif selection == "Set font size" then vim.cmd('Nekifoch close'); vim.cmd('Nekifoch set_size')"#;

    let lua_cmd = format!(
        "<cmd>lua local selection = vim.api.nvim_get_current_line(); {} {} {} {} end<CR>",
        check_font, show_installed_fonts, set_font_family, set_font_size
    );

    let menu_mappings = vec![
        ("<CR>", lua_cmd.as_str()),
        ("<Esc>", CLOSE_COMMAND),
        ("<BS>", "<Nop>"),
    ];

    set_keymaps(
        buf,
        menu_mappings,
        SetKeymapOpts::builder().noremap(true).silent(true).build(),
    )
}
