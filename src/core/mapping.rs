use nvim_oxi::{
    api::{
        create_autocmd,
        opts::{CreateAutocmdOpts, SetKeymapOpts},
        types::Mode,
        Buffer, Window,
    },
    Result as OxiResult,
};

use super::App;

pub fn set_keymaps_for_buffer(buf: &mut Buffer) -> OxiResult<()> {
    let opts = SetKeymapOpts::builder().noremap(true).silent(true).build();

    buf.set_keymap(Mode::Normal, "<CR>",
        r#":lua local font_name = vim.api.nvim_get_current_line(); local formatted_font_name = font_name:gsub('%s+', ''); vim.cmd('Nekifoch set_font ' .. formatted_font_name)<CR>"#,
        &opts)?;

    buf.set_keymap(Mode::Normal, "q", "<cmd>Nekifoch close<CR>", &opts)?;
    buf.set_keymap(Mode::Normal, "<Esc>", "<cmd>Nekifoch close<CR>", &opts)?;

    buf.set_keymap(Mode::Normal, "j", "gj", &opts)?;
    buf.set_keymap(Mode::Normal, "<Down>", "gj", &opts)?;

    buf.set_keymap(Mode::Normal, "k", "gk", &opts)?;
    buf.set_keymap(Mode::Normal, "<Up>", "gk", &opts)?;

    Ok(())
}

fn setup_enter_key_handler(buf: &Buffer, app: &mut App) -> OxiResult<()> {
    // Создаем опции автокоманды с указанием конкретного буфера
    let opts = CreateAutocmdOpts::builder().buffer(buf.clone()).build();

    // Создаем автокоманду для обработки нажатия Enter и вызова set_font_family
    create_autocmd(vec!["BufWinLeave"], &opts)?;

    // Вызов команды set_font
    let selected_line = nvim_oxi::api::get_current_line()?; // Получаем текущую строку
    app.handle_command("set_font", Some(&selected_line))?;

    Ok(())
}

fn setup_close_key_handler(buf: &Buffer, window: Window, app: &mut App) -> OxiResult<()> {
    // Опции автокоманды с указанием буфера
    let opts = CreateAutocmdOpts::builder().buffer(buf.clone()).build();

    // Обрабатываем нажатие 'q' или 'Esc' для закрытия окна
    create_autocmd(vec!["BufWinLeave"], &opts)?;

    app.handle_command("close", None)?;

    Ok(())
}
