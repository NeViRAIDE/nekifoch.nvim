use nvim_oxi::{
    api::{create_buf, create_user_command, err_writeln, open_win, types::*},
    Dictionary, Error as OxiError, Function, Result as OxiResult,
};

use crate::setup::{setup, CONFIG, WIN};

mod error;
mod setup;
mod utils;

pub fn open_window(_args: CommandArgs) -> OxiResult<()> {
    WIN.with(|win| {
        if win.borrow().is_some() {
            err_writeln("Window is already open");
            return Ok(());
        }

        let border = CONFIG.with(|c| c.borrow().border.clone());
        let win_border = match border.as_str() {
            "double" => WindowBorder::Double,
            "single" => WindowBorder::Single,
            "rounded" => WindowBorder::Rounded,
            _ => WindowBorder::None,
        };

        let buf = create_buf(false, true)?;
        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .height(10)
            .width(15)
            .title(WindowTitle::SimpleString("test win".into()))
            .border(win_border)
            .row(50)
            .col(50)
            .build();

        let mut win = win.borrow_mut();
        *win = Some(open_win(&buf, false, &config)?);

        Ok(())
    })
}

pub fn close_window(_args: CommandArgs) -> OxiResult<()> {
    WIN.with(|win| {
        if win.borrow().is_none() {
            err_writeln("Window is already closed");
            return Ok(());
        }

        let win = win.borrow_mut().take().unwrap();
        win.close(false).map_err(|e| e.into())
    })
}

#[nvim_oxi::plugin]
fn nekifoch() -> OxiResult<Dictionary> {
    create_user_command("NekifochOpenWin", open_window, &Default::default())?;
    create_user_command("NekifochCloseWin", close_window, &Default::default())?;

    let exports: Dictionary =
        Dictionary::from_iter::<[(&str, Function<Dictionary, Result<(), OxiError>>); 1]>([(
            "setup",
            Function::from_fn(|dict: Dictionary| -> Result<(), OxiError> { setup(dict) }),
        )]);

    Ok(exports)
}
