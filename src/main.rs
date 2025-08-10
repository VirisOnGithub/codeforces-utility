use std::{env, fs, process::Command};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        eprintln!("Usage: {} <url>", args[0]);
        return;
    }
    let url = &args[1];
    if url == "run" {
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
            Err(_) => eprintln!("No current problem file found. Please specify a problem URL."),
        }
    } else {
        let re = regex::Regex::new(r"^https://codeforces.com/problemset/problem/(\d+)/([A-F])$")
            .expect("Failed to compile regex");
        if let Some(caps) = re.captures(url) {
            let problem_id = &caps[1];
            let problem_type = &caps[2];
            let mut problem_name = format!("{problem_id}{problem_type}");
            fs::write(".curfile", &problem_name).expect("Failed to write current problem file");
            problem_name.push_str(".py");
            fs::File::create(&problem_name).expect("Failed to create file");
            Command::new("zed")
                .arg(&problem_name)
                .status()
                .expect("Failed to open editor");
        }
    }
}
