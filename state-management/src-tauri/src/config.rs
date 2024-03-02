use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use std::sync::Mutex;

const SETTINGS_FILENAME: &str = "settings.json";

#[cfg(target_os = "windows")]
fn get_config_root() -> PathBuf {
    let appdata = PathBuf::from(std::env::var("APPDATA").unwrap());
    appdata.join("takanori").join("myapp")
}

#[cfg(target_os = "linux, macos")]
fn get_config_root() -> PathBuf {
    let home = PathBuf::from(std::env::var("Home").unwrap());
    home.join(".takanori").join("myapp")
}

trait Config {
    fn write_file(&self) {}
    fn read_file(&mut self) {}
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
            theme: "dark".to_string(),
        }
    }
}

impl Config for Settings {
    fn write_file(&self) {
        let config_file = get_config_root().join(SETTINGS_FILENAME);
        if !config_file.parent().unwrap().exists() {
            fs::create_dir(config_file.parent().unwrap()).unwrap();
        }
        let serialized = serde_json::to_string(self).unwrap();
        let mut file = fs::File::create(config_file).unwrap();
        file.write_all(&serialized.as_bytes()).unwrap();
    }

    fn read_file(&mut self) {
        let config_file = get_config_root().join(SETTINGS_FILENAME);
        let input = fs::read_to_string(config_file).unwrap();
        let deserialized: Self = serde_json::from_str(&input).unwrap();
        let _ = mem::replace(self, deserialized);
    }
}

impl Settings {
    pub fn new() -> Self {
        let config_file = get_config_root().join(SETTINGS_FILENAME);
        if !config_file.exists() {
            Self::default()
        } else {
            let mut settings = Self::default();
            settings.read_file();
            settings
        }
    }

    pub fn set_language(&mut self, new_lang: String) {
        self.language = new_lang;
        self.write_file();
        println!("{:?}", self);
    }

    pub fn set_theme(&mut self, new_theme: String) {
        self.theme = new_theme;
        self.write_file();
        println!("{:?}", self);
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
        let mut settings = state.settings.lock().unwrap();
        settings.set_language(new_language);
        Ok(())
    }

    #[tauri::command]
    pub async fn set_theme(
        state: tauri::State<'_, AppState>,
        new_theme: String,
    ) -> Result<(), String> {
        let mut settings = state.settings.lock().unwrap();
        settings.set_theme(new_theme);
        Ok(())
    }

    #[tauri::command]
    pub async fn get_settings(state: tauri::State<'_, AppState>) -> Result<Settings, String> {
        let settings = state.settings.lock().unwrap().clone();
        Ok(settings)
    }
}
