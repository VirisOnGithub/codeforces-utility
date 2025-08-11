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
        if args.len() < 3 {
            eprintln!("Usage: {} editor <editor_name>", args[0]);
            return;
        }
        let new_editor = &args[2];
        data::set_editor(new_editor).unwrap_or_else(|err| {
            eprintln!("Failed to set editor: {err}");
        });
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
                            Command::new(&editor)
                                .arg(&problem_name)
                                .status()
                                .expect("Failed to open editor");
                        }
                        None => {
                            eprintln!("No editor set. Use 'cfcf editor <editor_name>' to set one.")
                        }
                    }
                }
            }
        }
    }
}
