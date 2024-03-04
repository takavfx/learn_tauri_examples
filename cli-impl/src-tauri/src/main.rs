// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use std::collections::HashMap;

use tauri::{api::cli::ArgData, Manager};
use tauri_plugin_log::LogTarget;

use config::Settings;

use crate::config::AppState;

#[derive(Debug)]
struct CliCommands {
    args: HashMap<String, ArgData>,
}

fn main() {
    let app_state = config::AppState::new();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout])
                .build(),
        )
        .manage(app_state)
        .setup(|app| {
            println!("{:?}", app.get_cli_matches());
            match app.get_cli_matches() {
                Ok(matches) => {
                    println!("{:?}", matches);
                    // let mut settings = app_state.settings.lock().unwrap();
                    // app.state()<AppState>
                }
                Err(_) => {}
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
