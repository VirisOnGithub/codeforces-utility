use enum_all_variants::AllVariants;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

const STATE_FILE: &str = "app_state.json";
const APP_STATE_NONE: AppState = AppState {
    editor: None,
    languages: None,
    current_problem: None,
    current_language: None,
    favourite_language: None,
};

fn cache_path(file: &str) -> PathBuf {
    std::env::home_dir()
        .unwrap()
        .join(".cache")
        .join("cfcf")
        .join(file)
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    editor: Option<Editor>,
    languages: Option<Vec<Language>>,
    current_problem: Option<String>,
    current_language: Option<Language>,
    favourite_language: Option<Language>,
}

#[derive(Serialize, Deserialize, AllVariants, Copy, Clone)]
pub enum Editor {
    Neovide,
    Neovim,
    Vim,
    VsCode,
    Zed,
}

impl fmt::Display for Editor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Editor::Neovide => "Neovide",
            Editor::Neovim => "Neovim",
            Editor::Vim => "Vim",
            Editor::VsCode => "VsCode",
            Editor::Zed => "Zed",
        };
        write!(f, "{name}")
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, AllVariants, PartialEq)]
pub enum Language {
    Pypy,
    Python,
    Cpp,
    C,
    Java,
    Rust,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Language::C => "C",
            Language::Cpp => "C++",
            Language::Java => "Java",
            Language::Pypy => "Python (PyPy)",
            Language::Python => "Python",
            Language::Rust => "Rust",
        };
        write!(f, "{name}")
    }
}

pub fn editor_command(editor: &Editor) -> &str {
    match editor {
        Editor::Neovide => "neovide",
        Editor::Neovim => "nvim",
        Editor::Vim => "vim",
        Editor::VsCode => "code",
        Editor::Zed => "zed",
    }
}

pub fn load_state() -> AppState {
    std::fs::create_dir_all(cache_path(STATE_FILE).parent().unwrap()).ok();
    if let Ok(data) = std::fs::read_to_string(cache_path(STATE_FILE)) {
        serde_json::from_str(&data).unwrap_or(APP_STATE_NONE)
    } else {
        APP_STATE_NONE
    }
}

pub fn save_state(state: &AppState) -> Result<(), std::io::Error> {
    let data = serde_json::to_string(state)?;
    std::fs::write(cache_path(STATE_FILE), data)?;
    Ok(())
}

pub fn get_editor() -> Option<Editor> {
    let state = load_state();
    state.editor
}

pub fn set_editor(editor: Editor) -> Result<(), std::io::Error> {
    let mut state = load_state();
    state.editor = Some(editor);
    save_state(&state)?;
    Ok(())
}

pub fn get_languages() -> Option<Vec<Language>> {
    let state = load_state();
    state.languages
}

pub fn set_languages(languages: Vec<Language>) -> Result<(), std::io::Error> {
    let mut state = load_state();
    state.languages = Some(languages);
    save_state(&state)?;
    Ok(())
}

pub fn get_current_problem() -> Option<String> {
    let state = load_state();
    state.current_problem
}

pub fn set_current_problem(problem: String) -> Result<(), std::io::Error> {
    let mut state = load_state();
    state.current_problem = Some(problem);
    save_state(&state)?;
    Ok(())
}

pub fn get_current_language() -> Option<Language> {
    let state = load_state();
    state.current_language
}

pub fn set_current_language(language: Language) -> Result<(), std::io::Error> {
    let mut state = load_state();
    state.current_language = Some(language);
    save_state(&state)?;
    Ok(())
}

pub fn get_favourite_language() -> Option<Language> {
    let state = load_state();
    state.favourite_language
}

pub fn set_favourite_language(language: Language) -> Result<(), std::io::Error> {
    let mut state = load_state();
    state.favourite_language = Some(language);
    save_state(&state)?;
    Ok(())
}
