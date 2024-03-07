// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use std::collections::HashMap;

use tauri::{api::cli::ArgData, Manager, State};
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

                    let new_theme = matches
                        .args
                        .get("theme")
                        .clone()
                        .expect("theme is not set.");

                    match new_theme {
                        None => None,
                        Some(x) => {
                            let app_sate: State<'_, AppState> = app.state();
                            let mut settings = app_sate.settings.lock().unwrap();
                            settings.set_theme(*x.value);
                        }
                    }
                }
                Err(_) => {}
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
