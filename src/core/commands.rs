use nvim_oxi::{
    api::{err_writeln, out_write},
    print, Result as OxiResult, String as NvimString,
};

use crate::utils::Utils;

use super::{buffer::BufferManager, App};

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

        if let Err(err) = app.float_window.open(&app.config, " Set font ", compatible) {
            out_write(NvimString::from(format!("Error opening window: {}", err)));
        }

        if let Some(window) = &app.float_window.window {
            BufferManager::configure_buffer(window)?;
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
