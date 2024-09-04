use nvim_oxi::{
    api::{
        create_buf, create_user_command, err_writeln, open_win, opts::CreateCommandOpts, types::*,
        Window,
    },
    Dictionary, Error as OxiError, Function, Result as OxiResult,
};

mod error;
mod setup;
// mod utils;

pub struct App {
    config: setup::Config,
    window: Option<Window>,
}

impl App {
    pub fn new(config: setup::Config) -> Self {
        App {
            config,
            window: None,
        }
    }

    pub fn open_window(&mut self) -> OxiResult<()> {
        if self.window.is_some() {
            err_writeln("Window is already open");
            return Ok(());
        }

        let win_border = match self.config.border.as_str() {
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

        self.window = Some(open_win(&buf, false, &config)?);

        Ok(())
    }

    pub fn close_window(&mut self) -> OxiResult<()> {
        if self.window.is_none() {
            err_writeln("Window is already closed");
            return Ok(());
        }

        if let Some(win) = self.window.take() {
            win.close(false).map_err(|e| e.into())
        } else {
            Ok(())
        }
    }

    pub fn setup(&mut self, dict: Dictionary) -> OxiResult<()> {
        let config = setup::Config::from_dict(dict);
        self.config = config;
        Ok(())
    }

    pub fn handle_command(&mut self, cmd: &str) -> OxiResult<()> {
        match cmd {
            "open" => self.open_window(),
            "close" => self.close_window(),
            _ => {
                err_writeln(&format!("Unknown command: {}", cmd));
                Ok(())
            }
        }
    }
}

fn complete_fn(arg_lead: String, cmd_line: String, cursor_pos: usize) -> Vec<String> {
    let mut completions = vec!["open".into(), "close".into()];
    completions.retain(|c: &String| c.starts_with(&arg_lead));
    completions
}

#[nvim_oxi::plugin]
fn nekifoch() -> OxiResult<Dictionary> {
    let config = setup::Config::default();
    let app = std::sync::Arc::new(std::sync::Mutex::new(App::new(config)));

    let complete_fn = Function::from_fn(move |args: (String, String, usize)| {
        let (arg_lead, _cmd_line, _cursor_pos) = args;
        complete_fn(arg_lead, _cmd_line, _cursor_pos)
    });

    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("nekifoch description")
        .complete(CommandComplete::CustomList(complete_fn))
        .nargs(CommandNArgs::ZeroOrOne)
        .build();

    let app_handle_cmd = std::sync::Arc::clone(&app);
    create_user_command(
        "Nekifoch",
        move |args: CommandArgs| {
            let mut app = app_handle_cmd.lock().unwrap();
            if let Some(arg) = args.args.as_deref() {
                app.handle_command(arg)
            } else {
                err_writeln("Missing argument: expected 'open' or 'close'");
                Ok(())
            }
        },
        &opts,
    )?;

    let app_setup = std::sync::Arc::clone(&app);
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
