use std::env;
mod data;
mod fun;

pub const PROGRAM_NAME: &str = "cfc";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!("Usage: {} <command>", args[0]);
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "editor" => {
            fun::set_editor();
        }
        "lang" => {
            fun::set_languages();
        }
        "run" => match data::get_current_problem() {
            Some(curprob) => fun::run_problem(curprob.as_str()),
            None => eprintln!("No current problem file found. Please specify a problem URL."),
        },
        "clear_cache" => {
            if let Err(e) = data::clear_cache() {
                eprintln!("Failed to clear cache: {e}");
            }
        }
        _ => {
            let re =
                regex::Regex::new(r"^https://codeforces.com/problemset/problem/(\d+)/([A-F])$")
                    .expect("Failed to compile regex");
            if let Some(caps) = re.captures(command) {
                let problem_id = &caps[1];
                let problem_type = &caps[2];
                let problem_name = format!("{problem_id}{problem_type}");
                fun::create_problem(problem_name);
            } else {
                eprintln!(
                    "Invalid command.\n\
                    Usage:\n\
                    \t'{PROGRAM_NAME} editor' to set an editor\n\
                    \t'{PROGRAM_NAME} lang' to set languages\n\
                    \t'{PROGRAM_NAME} run' to run the current problem\n\
                    \tOr provide a valid Codeforces problem URL."
                );
            }
        }
    }
}
