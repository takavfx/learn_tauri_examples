// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::sync::Mutex;
use std::{fs, io::Write};

use serde::{Deserialize, Serialize};
use tauri::{api::file, App};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
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

impl Settings {
    pub fn set_language(&self, new_lang: String) {
        self.language = new_lang;
    }
}

#[derive(Debug)]
struct AppState {
    settings: Mutex<Settings>,
    settings_file: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: Mutex::from(Settings::default()),
            settings_file: "settings.json".to_string(),
        }
    }
}

impl AppState {
    pub fn set_settings_file(&self, file_name: String) {
        self.settings_file = file_name;
    }

    pub fn write_settings_file(&self) {
        let settings = self.settings.lock().unwrap();
        let serialized = serde_json::to_string(&*settings).unwrap();

        let mut file = fs::File::create(self.settings_file).unwrap();
        file.write_all(serialized.as_bytes());
    }

    pub fn read_settings_file(&self) {
        let input = fs::read_to_string(self.settings_file).unwrap();
        let deserialized: Settings = serde_json::from_str(&input).unwrap();

        let mut settings = self.settings.lock().unwrap();
        *settings = deserialized;
    }
}

#[test]
fn test_init_settings() {
    let state = AppState::default();
    println!("{:?}", state);
}

// remember to call `.manage(AppState::default())`
// #[tauri::command]
// async fn init_app_state(state: tauri::State<'_, AppState>) -> Result<(), String> {
//     state.write_settings_file();
//     Ok(())
// }

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
