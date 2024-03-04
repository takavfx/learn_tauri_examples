use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_theme(&mut self, new_theme: String) {
        self.theme = new_theme;
    }
}

#[derive(Debug)]
pub struct AppState {
    pub settings: Mutex<Settings>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            settings: Mutex::from(Settings::default()),
        }
    }
}

pub mod commands {
    use super::*;

    // remember to call `.manage(MyState::default())`
    #[tauri::command]
    async fn set_theme(state: tauri::State<'_, AppState>, new_theme: String) -> Result<(), String> {
        let mut settings = state.settings.lock().unwrap();
        settings.set_theme(new_theme);
        Ok(())
    }

    #[tauri::command]
    async fn get_settings(state: tauri::State<'_, AppState>) -> Result<Settings, String> {
        let settings = state.settings.lock().unwrap().clone();
        Ok(settings)
    }
}
