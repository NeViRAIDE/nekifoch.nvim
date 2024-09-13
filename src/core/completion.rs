use std::sync::{Arc, Mutex};

use nvim_oxi::Function;

use crate::utils::Utils;

pub fn completion() -> Function<(String, String, usize), Vec<String>> {
    let compatible_fonts_cache = Arc::new(Mutex::new(None));

    Function::from_fn({
        let compatible_fonts_cache = Arc::clone(&compatible_fonts_cache);
        move |args: (String, String, usize)| {
            let (arg_lead, cmd_line, cursor_pos) = args;

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

            let command = args_after_command.first().unwrap_or(&"");

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

                    let search_term = arg_lead.to_lowercase();

                    let mut filtered_fonts: Vec<String> = fonts
                        .iter()
                        .filter(|(formatted, _)| {
                            let formatted_lower = formatted.to_lowercase();
                            formatted_lower.contains(&search_term)
                        })
                        .map(|(formatted, _)| formatted.clone())
                        .collect();

                    filtered_fonts.sort();

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
    })
}
