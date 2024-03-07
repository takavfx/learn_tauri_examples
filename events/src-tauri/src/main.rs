// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let id = app.listen_global("click", |event| {
                println!("got `click` event with payload {:?}", event.payload());
            });

            match env::var("TAURI_UNLISTEN") {
                Ok(val) => {
                    let value: u8 = val.parse().unwrap();
                    if value == 1 {
                        app.unlisten(id);
                    }
                }
                Err(_err) => (),
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
