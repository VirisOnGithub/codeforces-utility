use std::{env, fs, process::Command};
mod data;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!("Usage: {} <command>", args[0]);
        return;
    }
    let command = &args[1];
    if command == "editor" {
        let editors = vec!["Neovide", "Neovim", "Vim", "VsCode", "Zed"];
        let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Choose your editor")
            .default(0)
            .items(&editors)
            .interact()
            .expect("Failed to select editor");
        let new_editor = match selection {
            0 => data::Editor::Neovide,
            1 => data::Editor::Neovim,
            2 => data::Editor::Vim,
            3 => data::Editor::VsCode,
            4 => data::Editor::Zed,
            _ => {
                eprintln!("Invalid selection");
                return;
            }
        };
        match data::set_editor(new_editor) {
            Ok(_) => {
                println!(
                    "Editor set to: \x1b[32m\x1b[1m{}\x1b[0m",
                    editors[selection]
                );
            }
            Err(e) => {
                eprintln!("Failed to set editor: {e}");
            }
        }
    } else {
        match command.as_str() {
            "run" => {
                let curprob = fs::File::open(".curfile");
                match curprob {
                    Ok(file) => {
                        let mut contents = String::new();
                        use std::io::Read;
                        file.take(100)
                            .read_to_string(&mut contents)
                            .expect("Failed to read file");
                        let problem_name = contents.trim();
                        Command::new("pypy")
                            .arg(format!("{problem_name}.py"))
                            .status()
                            .expect("Failed to run the problem");
                    }
                    Err(_) => {
                        eprintln!("No current problem file found. Please specify a problem URL.")
                    }
                }
            }
            _ => {
                let re =
                    regex::Regex::new(r"^https://codeforces.com/problemset/problem/(\d+)/([A-F])$")
                        .expect("Failed to compile regex");
                if let Some(caps) = re.captures(command) {
                    let problem_id = &caps[1];
                    let problem_type = &caps[2];
                    let mut problem_name = format!("{problem_id}{problem_type}");
                    fs::write(".curfile", &problem_name)
                        .expect("Failed to write current problem file");
                    problem_name.push_str(".py");
                    fs::File::create(&problem_name).expect("Failed to create file");
                    match data::get_editor() {
                        Some(editor) => {
                            Command::new(data::editor_command(&editor))
                                .arg(&problem_name)
                                .status()
                                .expect("Failed to open editor");
                        }
                        None => {
                            eprintln!("No editor set. Use 'cfcf editor' to set one.")
                        }
                    }
                }
            }
        }
    }
}
