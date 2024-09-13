use std::sync::{Arc, Mutex};

use nvim_oxi::{
    api::{create_user_command, opts::CreateCommandOpts, types::*},
    Dictionary, Error as OxiError, Function, Result as OxiResult,
};

use setup::Config;

use core::{completion::completion, App};

mod core;
mod error;
mod setup;
mod utils;

#[nvim_oxi::plugin]
fn nekifoch() -> OxiResult<Dictionary> {
    let config = Config::default();

    let app = Arc::new(Mutex::new(App::new(config)));

    let app_handle_cmd = Arc::clone(&app);

    let nekifoch_cmd = move |args: CommandArgs| {
        let mut app = app_handle_cmd.lock().unwrap();

        let binding = match args.args {
            Some(a) => a,
            None => {
                nvim_oxi::api::err_writeln("Missing arguments. Expected action.");
                return Ok(());
            }
        };
        let mut split_args = binding.split_whitespace();
        let action = split_args.next().unwrap_or("");
        let argument = split_args.next();

        app.handle_command(action, argument)
    };

    let opts = CreateCommandOpts::builder()
        // .bang(true)
        .desc("Nekifoch command")
        .complete(CommandComplete::CustomList(completion()))
        .nargs(CommandNArgs::OneOrMore)
        .build();

    create_user_command("Nekifoch", nekifoch_cmd, &opts)?;

    let app_setup = Arc::clone(&app);
    let exports: Dictionary =
        Dictionary::from_iter::<[(&str, Function<Dictionary, Result<(), OxiError>>); 1]>([(
            "setup",
            Function::from_fn(move |dict: Dictionary| -> OxiResult<()> {
                let mut app = app_setup.lock().unwrap();
                app.setup(dict)
            }),
        )]);

    Ok(exports)
}
