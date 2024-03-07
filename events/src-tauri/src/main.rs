// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::Manager;

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("click", |event| {
                println!("got `click` event with payload {:?}", event.payload());
            });

            app.unlisten(id);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
