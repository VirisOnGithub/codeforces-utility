use crate::{
    PROGRAM_NAME,
    data::{self, Editor, Language},
};
use dialoguer::{MultiSelect, Select, theme::ColorfulTheme};
use std::{fs, process::Command};

pub fn set_editor() {
    let editors = Editor::all_variants();
    let editors_str = editors
        .iter()
        .map(|editor| editor.to_string())
        .collect::<Vec<String>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your editor")
        .default(0)
        .items(&editors_str)
        .interact()
        .expect("Failed to select editor");
    let new_editor = editors[selection];
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
}

pub fn set_languages() {
    let languages = Language::all_variants();
    let languages_str = languages
        .iter()
        .map(|lang| lang.to_string())
        .collect::<Vec<String>>();
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your language")
        .items(&languages_str)
        .interact()
        .expect("Failed to select language");
    let selected_languages: Vec<Language> = selection.iter().map(|&i| languages[i]).collect();
    let selected_languages_str = selected_languages
        .iter()
        .map(|lang| lang.to_string())
        .collect::<Vec<String>>();
    match data::set_languages(selected_languages.clone()) {
        Ok(_) => {
            println!(
                "Languages set to: \x1b[32m\x1b[1m{}\x1b[0m",
                selected_languages_str.join(", ")
            );
        }
        Err(e) => {
            eprintln!("Failed to set languages: {e}");
            return;
        }
    }
    if !selected_languages.is_empty() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your default language")
            .default(0)
            .items(&selected_languages_str)
            .interact()
            .expect("Failed to select default language");
        if let Err(e) = data::set_favourite_language(selected_languages[selection]) {
            eprintln!("Failed to set current language: {e}");
        }
    }
}

pub fn create_problem(problem_name: String) {
    if let Err(e) = data::set_current_problem(problem_name.clone()) {
        eprintln!("Failed to set current problem: {e}");
        return;
    }

    let languages_opt = data::get_languages();
    let languages = match languages_opt {
        Some(ref langs) if !langs.is_empty() => langs,
        _ => {
            eprintln!("No languages set. Use '{PROGRAM_NAME} lang' to set languages.");
            return;
        }
    };

    let selected_language: Language;

    if languages.len() > 1 {
        let languages_str = languages
            .iter()
            .map(|lang| lang.to_string())
            .collect::<Vec<String>>();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your language")
            .default(
                languages
                    .iter()
                    .position(|l| l == &data::get_favourite_language().unwrap_or(languages[0]))
                    .unwrap_or(0),
            )
            .items(&languages_str)
            .interact()
            .expect("Failed to select language");
        selected_language = languages[selection];
        if let Err(e) = data::set_current_language(selected_language) {
            eprintln!("Failed to set current language: {e}");
        }
    } else {
        selected_language = languages[0];
    }

    create_problem_file(&problem_name, &selected_language);
    if let Some(editor) = data::get_editor() {
        Command::new(data::editor_command(&editor))
            .arg(get_filename(&problem_name, &selected_language))
            .status()
            .expect("Failed to open the problem file in the editor");
    } else {
        eprintln!("No editor set. Use '{PROGRAM_NAME} editor' to set one.");
    }
}

pub fn create_problem_file(problem_name: &str, language: &Language) {
    match language {
        Language::Pypy | Language::Python => create_problem_file_with_extension(problem_name, "py"),
        Language::C | Language::Cpp => create_problem_file_with_extension(problem_name, "cpp"),
        Language::Java => create_problem_file_with_extension(problem_name, "java"),
        Language::Rust => {
            Command::new("cargo")
                .arg("new")
                .arg(format!("r{}", problem_name.to_lowercase()))
                .arg("--vcs")
                .arg("none")
                .status()
                .expect("Failed to create Rust project");
        }
    }
}

fn create_problem_file_with_extension(problem_name: &str, extension: &str) {
    let file_name = format!("{problem_name}.{extension}");
    if fs::metadata(&file_name).is_ok() {
        eprintln!("File '{file_name}' already exists. Skipping creation.");
        return;
    }
    if fs::File::create(&file_name).is_ok() {
        println!("Problem file created: \x1b[32m\x1b[1m{file_name}\x1b[0m");
    } else {
        eprintln!("Failed to create file '{file_name}'");
    }
}

fn get_filename(problem_name: &str, language: &Language) -> String {
    match language {
        Language::C => format!("{problem_name}.c"),
        Language::Cpp => format!("{problem_name}.cpp"),
        Language::Java => format!("{problem_name}.java"),
        Language::Pypy | Language::Python => format!("{problem_name}.py"),
        Language::Rust => {
            format!("r{}/src/main.rs", problem_name.to_lowercase())
        }
    }
}

pub fn run_problem(problem_name: &str) {
    let current_language = data::get_current_language();
    if let Some(language) = current_language {
        match language {
            Language::C | Language::Cpp => {
                let file_ext = if language == Language::C { "c" } else { "cpp" };
                let source_file = format!("{problem_name}.{file_ext}");
                let compiler = if language == Language::C {
                    "gcc"
                } else {
                    "g++"
                };
                Command::new(compiler)
                    .arg(&source_file)
                    .arg("-o")
                    .arg(problem_name)
                    .status()
                    .expect("Failed to compile C/C++ code");
                Command::new(format!("./{problem_name}"))
                    .status()
                    .expect("Failed to run the compiled C/C++ code");
            }
            Language::Pypy | Language::Python => {
                let file_name = format!("{problem_name}.py");
                let interpreter = if language == Language::Pypy {
                    "pypy"
                } else {
                    "python3"
                };
                Command::new(interpreter)
                    .arg(&file_name)
                    .status()
                    .expect("Failed to run the Python/PyPy code");
            }
            Language::Java => {
                let file_name = format!("{problem_name}.java");
                Command::new("javac")
                    .arg(&file_name)
                    .status()
                    .expect("Failed to compile Java code");
                Command::new("java")
                    .arg(problem_name)
                    .status()
                    .expect("Failed to run the Java code");
            }
            Language::Rust => {
                Command::new("cargo")
                    .arg("run")
                    .current_dir(format!("r{}", problem_name.to_lowercase()))
                    .status()
                    .expect("Failed to run the Rust project");
            }
        }
    } else {
        eprintln!(
            "No current language set. This should not happen. Try to recreate the problem file."
        );
    }
}
