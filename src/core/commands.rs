use nvim_oxi::{
    api::{err_writeln, out_write},
    print, Result as OxiResult, String as NvimString,
};

use crate::utils::Utils;

use super::App;

pub fn get_current_font(app: &mut App) -> OxiResult<()> {
    let fonts = Utils::get(&app.config)?;
    print!(
        "\nFont family: {:?}\nFont size: {:?}\n",
        fonts["font"], fonts["size"]
    );
    Ok(())
}

pub fn set_font_family(app: &mut App, arg: Option<&str>) -> OxiResult<()> {
    if let Some(font_family) = arg {
        Utils::replace_font_family(&app.config, font_family)?;
        out_write(NvimString::from(format!(
            "Font family set to {}",
            font_family
        )));
    } else {
        let installed_fonts = Utils::list_installed_fonts();
        let mut compatible: Vec<String> =
            Utils::compare_fonts_with_kitty_list_fonts(installed_fonts)
                .values()
                .cloned()
                .collect();
        compatible.sort();

        if let Err(err) = app.open_window(" Set font ", compatible) {
            out_write(NvimString::from(format!("Error opening window: {}", err)));
        }

        if let Some(window) = &app.window {
            let buf_opts = nvim_oxi::api::opts::OptionOpts::builder()
                .scope(nvim_oxi::api::opts::OptionScope::Local) // Указываем локальную область применения
                .win(window.clone()) // Применяем опции к конкретному окну
                .build();

            nvim_oxi::api::set_option_value("number", false, &buf_opts)?;
            nvim_oxi::api::set_option_value("relativenumber", false, &buf_opts)?;
        } else {
            err_writeln("Window is not open.");
        }
    }
    Ok(())
}

pub fn set_font_size(app: &mut App, arg: Option<&str>) -> OxiResult<()> {
    if let Some(size_str) = arg {
        if let Ok(size) = size_str.parse::<f32>() {
            Utils::replace_font_size(&app.config, size)?;
            out_write(NvimString::from(format!("Font size set to {}", size)));
        } else {
            err_writeln("Invalid font size argument for set_size action");
        }
    } else {
        err_writeln("Missing font size argument for set_size action");
    }
    Ok(())
}

pub fn get_fonts_list() -> OxiResult<()> {
    let installed_fonts = Utils::list_installed_fonts();
    let compatible = Utils::compare_fonts_with_kitty_list_fonts(installed_fonts);
    print!("Available fonts:");
    for font in compatible.values() {
        print!("  - {font}");
    }
    Ok(())
}
