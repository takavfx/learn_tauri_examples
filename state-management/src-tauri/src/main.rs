// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{api::file, generate_handler, App};

mod config;

fn main() {
    let app_state = config::AppState::new();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(generate_handler![
            config::commands::set_language,
            config::commands::set_theme,
            config::commands::get_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
