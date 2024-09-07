use std::sync::{Arc, Mutex};

use nvim_oxi::{
    api::{create_user_command, opts::CreateCommandOpts, types::*},
    Dictionary, Error as OxiError, Function, Result as OxiResult,
};

use setup::Config;

use self::{core::App, utils::Utils};

mod core;
mod error;
mod setup;
mod utils;

#[nvim_oxi::plugin]
fn nekifoch() -> OxiResult<Dictionary> {
    let config = Config::default();

    let app = Arc::new(Mutex::new(App::new(config)));

    let notify_opts = nvim_oxi::api::opts::NotifyOpts::builder().build();

    let compatible_fonts_cache = Arc::new(Mutex::new(None));

    // TODO: move to new module
    let complete_fn = Function::from_fn({
        let compatible_fonts_cache = Arc::clone(&compatible_fonts_cache);
        move |args: (String, String, usize)| {
            let (arg_lead, cmd_line, cursor_pos) = args;

            nvim_oxi::api::notify(
                &format!("ARG LEAD: {arg_lead}, CMD_LINE: {cmd_line}, CURSOR_POS: {cursor_pos}"),
                LogLevel::Info,
                &notify_opts,
            );

            let split_cmd_line: Vec<&str> = cmd_line.split_whitespace().collect();
            let args_after_command = &split_cmd_line[1..];

            let mut current_arg_index = 0;

            for (index, &arg) in args_after_command.iter().enumerate() {
                if let Some(start_pos) = cmd_line.find(arg) {
                    let end_pos = start_pos + arg.len();
                    if cursor_pos >= start_pos && cursor_pos <= end_pos {
                        current_arg_index = index;
                        break;
                    }
                }
            }

            nvim_oxi::api::notify(
                &format!("CUR ARG IND: {current_arg_index}"),
                LogLevel::Info,
                &notify_opts,
            );

            let command = args_after_command.first().unwrap_or(&"");

            nvim_oxi::api::notify(&format!("COMMAND: {command}"), LogLevel::Info, &notify_opts);

            nvim_oxi::api::notify(
                &format!("CUR ARG IND: {current_arg_index}"),
                LogLevel::Info,
                &notify_opts,
            );

            match command {
                &"set_font" => {
                    let mut fonts_cache = compatible_fonts_cache.lock().unwrap();

                    if fonts_cache.is_none() {
                        let installed_fonts = Utils::list_installed_fonts();
                        let compatible_fonts =
                            Utils::compare_fonts_with_kitty_list_fonts(installed_fonts);
                        *fonts_cache = Some(compatible_fonts);
                    }

                    let fonts = fonts_cache.as_ref().unwrap();

                    // Преобразуем arg_lead в нижний регистр
                    let search_term = arg_lead.to_lowercase();

                    // Фильтруем шрифты, проверяя, содержит ли отформатированное имя шрифта искомую подстроку
                    let mut filtered_fonts: Vec<String> = fonts
                        .iter()
                        .filter(|(formatted, _)| {
                            // Преобразуем отформатированное имя шрифта в нижний регистр
                            let formatted_lower = formatted.to_lowercase();
                            // Проверяем, содержит ли имя шрифта искомую подстроку
                            formatted_lower.contains(&search_term)
                        })
                        .map(|(formatted, _)| formatted.clone()) // Клонируем ключи
                        .collect();

                    // Сортируем шрифты
                    filtered_fonts.sort();

                    // Возвращаем отсортированный список шрифтов для автодополнения
                    filtered_fonts
                }
                &"list" | &"check" | &"set_size" => {
                    vec![]
                }
                _ => {
                    if current_arg_index > 0 {
                        vec![]
                    } else {
                        let completions = vec![
                            "check".into(),
                            "set_font".into(),
                            "set_size".into(),
                            "list".into(),
                        ];
                        completions
                            .into_iter()
                            .filter(|c: &String| c.starts_with(&arg_lead))
                            .collect::<Vec<_>>()
                    }
                }
            }
        }
    });

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
        .bang(true)
        .desc("Nekifoch command")
        .complete(CommandComplete::CustomList(complete_fn))
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
