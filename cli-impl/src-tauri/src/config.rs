use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    dark_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { dark_mode: false }
    }
}

impl Settings {
    pub fn set_dark_mode(&mut self, switch: bool) {
        self.dark_mode = switch;
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
    pub async fn set_dark_mode(
        state: tauri::State<'_, AppState>,
        switch: bool,
    ) -> Result<(), String> {
        let mut settings: std::sync::MutexGuard<'_, Settings> = state.settings.lock().unwrap();
        settings.set_dark_mode(switch);
        Ok(())
    }

    #[tauri::command]
    pub async fn get_settings(state: tauri::State<'_, AppState>) -> Result<Settings, String> {
        let settings = state.settings.lock().unwrap().clone();
        Ok(settings)
    }
}
