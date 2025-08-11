use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const STATE_FILE: &str = "app_state.json";

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
}

#[derive(Serialize, Deserialize)]
pub enum Editor {
    Neovide,
    Neovim,
    Vim,
    VsCode,
    Zed,
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
        serde_json::from_str(&data).unwrap_or(AppState { editor: None })
    } else {
        AppState { editor: None }
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
