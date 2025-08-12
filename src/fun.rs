use crate::data::{self, Editor};
use dialoguer::{Select, theme::ColorfulTheme};
use std::process::Command;

pub fn set_editor() {
    let editors = vec!["Neovide", "Neovim", "Vim", "VsCode", "Zed"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your editor")
        .default(0)
        .items(&editors)
        .interact()
        .expect("Failed to select editor");
    let new_editor = match selection {
        0 => Editor::Neovide,
        1 => Editor::Neovim,
        2 => Editor::Vim,
        3 => Editor::VsCode,
        4 => Editor::Zed,
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
}

pub fn run_problem(problem_name: &str) {
    Command::new("pypy")
        .arg(format!("{problem_name}.py"))
        .status()
        .expect("Failed to run the problem");
}
