// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use std::any::Any;

use config::AppState;
use tauri::{generate_handler, Manager, State};
use tauri_plugin_log::{LogTarget, RotationStrategy, TimezoneStrategy};
use windows_sys::Win32::System::Console::AllocConsole;

use log::{debug, LevelFilter};

fn main() {
    let app_state = config::AppState::new();

    let builder = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout, LogTarget::LogDir])
                .rotation_strategy(RotationStrategy::KeepAll)
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .level(LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(generate_handler![
            config::commands::get_settings,
            config::commands::set_dark_mode
        ])
        .manage(app_state)
        .setup(|app| {
            debug!("{:?}", app.get_cli_matches());
            match app.get_cli_matches() {
                Ok(matches) => {
                    debug!("{:?}", matches);

                    // --help の表示
                    if let Some(x) = matches.args.get("help").clone() {
                        println!("{}", x.value.as_str().expect("The value is not str type."));
                        std::process::exit(0);
                    }

                    // --version の表示
                    if let Some(_) = matches.args.get("version").clone() {
                        println!(
                            "{}, Version: {}",
                            app.config()
                                .as_ref()
                                .package
                                .product_name
                                .clone()
                                .expect("To get product name is failed."),
                            app.config()
                                .as_ref()
                                .package
                                .version
                                .clone()
                                .expect("To get version is failed.")
                        );
                        std::process::exit(0);
                    }

                    let mut is_only_gui = false;
                    for &key in matches.args.keys() {
                        if let Some(&value) = matches.args.get(key.as_str()) {
                            match value.type_id() {
                                Bool => {
                                    if value == true {
                                        is_only_gui = false;
                                    } else {
                                        is_only_gui = true;
                                    }
                                }
                            }
                        }
                    }
                    if is_only_gui {
                        unsafe {
                            AllocConsole();
                        }
                    }

                    // --dark 時の処理
                    if let Some(x) = matches.args.get("dark").clone() {
                        let app_state: State<'_, AppState> = app.state();
                        debug!("{:?}", app_state.settings.lock().unwrap());
                        let mut settings = app_state.settings.lock().unwrap();
                        settings
                            .set_dark_mode(x.value.as_bool().expect("The value is not bool type."));
                        debug!("{:?}", settings);
                    } else {
                        unsafe {
                            AllocConsole();
                        }
                    }
                }
                Err(e) => println!("{}", e.to_string()),
            }
            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
