#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

pub mod utils;

use utils::{get_activities, get_user, greet, read_config, save_user, start_activity};

fn main() {
    let user = read_config();

    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            let _ = win.move_window(Position::TopRight);

            Ok(())
        })
        .manage(user)
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user,
            save_user,
            get_activities,
            start_activity
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
