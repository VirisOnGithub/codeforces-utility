use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const STATE_FILE: &str = "app_state.json";
const APP_STATE_NONE: AppState = AppState {
    editor: None,
    language: None,
    current_problem: None,
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
    language: Option<Language>,
    current_problem: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Editor {
    Neovide,
    Neovim,
    Vim,
    VsCode,
    Zed,
}

#[derive(Serialize, Deserialize)]
pub enum Language {
    Pypy,
    Python,
    Cpp,
    C,
    Java,
    Rust,
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

pub fn get_language() -> Option<Language> {
    let state = load_state();
    state.language
}

pub fn set_language(language: Language) -> Result<(), std::io::Error> {
    let mut state = load_state();
    state.language = Some(language);
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
