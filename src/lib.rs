use std::sync::{Arc, Mutex};

use nvim_oxi::{
    api::{create_user_command, err_writeln, opts::CreateCommandOpts, types::*},
    Dictionary, Error as OxiError, Function, Result as OxiResult,
};

use setup::Config;

use core::{command::Command, completion::completion, App};

mod core;
mod error;
mod setup;
mod utils;

#[nvim_oxi::plugin]
fn nekifoch() -> OxiResult<Dictionary> {
    let config = Config::default();

    let app = Arc::new(Mutex::new(App::new(config)));

    let nekifoch_cmd = {
        let app_handle_cmd = Arc::clone(&app);

        move |args: CommandArgs| -> OxiResult<()> {
            let binding = match args.args {
                Some(a) => a,
                None => {
                    nvim_oxi::api::err_writeln("Missing arguments. Expected action.");
                    return Ok(());
                }
            };

            let mut split_args = binding.split_whitespace();
            let action = split_args.next().unwrap_or("").to_string();
            let argument = split_args.next().map(|s| s.to_string());

            let command = Command::from_str(&action, argument.as_deref());

            if let Some(command) = command {
                app_handle_cmd.lock().unwrap().handle_command(command)?;
            } else {
                err_writeln(&format!("Unknown command: {}", action));
            };

            Ok(())
        }
    };

    let opts = CreateCommandOpts::builder()
        .desc("Nekifoch command")
        .complete(CommandComplete::CustomList(completion()))
        .nargs(CommandNArgs::Any)
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
