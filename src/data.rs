use enum_all_variants::AllVariants;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

use crate::fun::get_filename;

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
    C,
    Cpp,
    Java,
    Pypy,
    Python,
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

impl Language {
    pub fn name_utf8(&self) -> String {
        match self {
            Language::C => "c".to_string(),
            Language::Cpp => "cpp".to_string(),
            Language::Java => "java".to_string(),
            Language::Pypy => "pypy".to_string(),
            Language::Python => "python".to_string(),
            Language::Rust => "rs".to_string(),
        }
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

#[allow(dead_code)]
pub fn save_template(template: &str, language: Language) -> Result<(), std::io::Error> {
    create_template_dir()?;
    let template_path = cache_path("templates").join(format!("template.{}", language.name_utf8()));
    std::fs::write(template_path, template)?;
    Ok(())
}

pub fn create_template_dir() -> Result<(), std::io::Error> {
    std::fs::create_dir_all(cache_path("templates"))?;
    Ok(())
}

pub fn get_template_path(language: Language) -> PathBuf {
    cache_path("templates").join(format!("template.{}", language.name_utf8()))
}

pub fn load_template(language: &Language) -> Result<String, std::io::Error> {
    let template_name = format!("template.{}", language.name_utf8());
    let template_path = cache_path("templates").join(template_name);
    std::fs::read_to_string(template_path)
}

pub fn clear_cache() -> Result<(), std::io::Error> {
    save_state(&APP_STATE_NONE)
}

pub(crate) fn apply_template(
    problem_name: &str,
    selected_language: &Language,
) -> Result<(), std::io::Error> {
    let template = load_template(selected_language)?;
    let problem_path = std::env::current_dir()?.join(get_filename(problem_name, selected_language));
    std::fs::write(problem_path, template)?;
    Ok(())
}
