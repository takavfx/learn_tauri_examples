use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::utils::config;

#[cfg(target_os = "windows")]
fn get_config_root() -> PathBuf {
    let appdata = PathBuf::from(std::env::var("APPDAT").unwrap());
    appdata.join("takanori").join("myapp")
}

#[cfg(target_os = "linux, macos")]
fn get_config_root() -> PathBuf {
    let home = PathBuf::from(std::env::var("Home").unwrap());
    home.join(".takanori").join("myapp")
}

trait Config {
    fn write_file(&self) {}
    fn read_file(&mut self) -> Self;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    language: String,
    theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            theme: "jp".to_string(),
        }
    }
}

impl Config for Settings {
    fn write_file(&self) {
        let config_file = get_config_root().join("settings.json");
        let serialized = serde_json::to_string(self).unwrap();
        let mut file = fs::File::create(&config_file).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn read_file(&mut self) -> Settings {
        let config_file = get_config_root().join("settings.json");
        let input = fs::read_to_string(&config_file).unwrap();
        let deserialized: Settings = serde_json::from_str(&input).unwrap();

        deserialized
    }
}

impl Settings {
    pub fn new() -> Self {
        let config_file = get_config_root().join("settings.json");
        if !config_file.exists() {
            return Self::default();
        } else {
            return Self::default().read_file();
        }
    }

    pub fn set_language(&mut self, new_lang: String) {
        self.language = new_lang;
        self.write_file();
    }

    pub fn set_theme(&mut self, new_theme: String) {
        self.theme = new_theme;
        self.write_file();
    }
}

#[derive(Debug)]
pub struct AppState {
    settings: Mutex<Settings>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            settings: Mutex::from(Settings::new()),
        }
    }
}

pub mod commands {
    use super::*;

    #[tauri::command]
    pub async fn set_language(
        state: tauri::State<'_, AppState>,
        new_language: String,
    ) -> Result<(), String> {
        state.settings.lock().unwrap().set_language(new_language);
        Ok(())
    }

    #[tauri::command]
    pub async fn set_theme(
        state: tauri::State<'_, AppState>,
        new_theme: String,
    ) -> Result<(), String> {
        state.settings.lock().unwrap().set_theme(new_theme);
        Ok(())
    }

    #[tauri::command]
    pub async fn get_settings(state: tauri::State<'_, AppState>) -> Result<Settings, String> {
        let settings = state.settings.lock().unwrap().clone();
        Ok(settings)
    }
}
