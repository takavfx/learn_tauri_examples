// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Utc;
use cron::Schedule;
use serde::Serialize;
use std::{env, str::FromStr, thread::sleep, time::Duration};
use tauri::Manager;

#[derive(Clone, Serialize)]
struct CountEventPayload {
    count: u32,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Click event ==========
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
                _ => (),
            }

            // Count Event ==========
            let app_handle = app.app_handle();

            // そのままループを作るとメイン処理が固まるので、tauri::async_runtime
            // で非同期ランタイムを作成
            let _count_handle = tauri::async_runtime::spawn(async move {
                // cron 式で3秒ごとのイベントをスケジュールする。
                let schedule = Schedule::from_str("0/3 * * * * *").unwrap();
                let mut count: u32 = 0;
                let mut next_tick = schedule.upcoming(Utc).next().unwrap();

                loop {
                    let now = Utc::now();

                    // 1 カウントする
                    if now >= next_tick {
                        next_tick = schedule.upcoming(Utc).next().unwrap();
                        count += 1;

                        // イベントを emit。
                        let result =
                            app_handle.emit_all("count_event", CountEventPayload { count: count });
                        match result {
                            Err(ref err) => println!("{:?}", err),
                            _ => (),
                        }
                    }

                    sleep(Duration::from_secs(std::cmp::min(
                        (next_tick - now).num_seconds() as u64,
                        60,
                    )));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
