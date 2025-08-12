use std::{env, fs, process::Command};
mod data;
mod fun;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!("Usage: {} <command>", args[0]);
        return;
    }
    let command = &args[1];
    if command == "editor" {
        fun::set_editor();
    } else {
        match command.as_str() {
            "run" => {
                let curprob = data::get_current_problem();
                if let Some(curprob) = curprob {
                    fun::run_problem(curprob.as_str());
                } else {
                    eprintln!("No current problem file found. Please specify a problem URL.")
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
                    data::set_current_problem(problem_name.clone())
                        .expect("Failed to set current problem");
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
                } else {
                    eprintln!(
                        "Invalid command. Use 'cfcf editor' to set an editor or 'cfcf run' to run the current problem."
                    );
                }
            }
        }
    }
}
