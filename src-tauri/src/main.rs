#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, State};
use tauri_plugin_positioner::{Position, WindowExt};

pub mod utils;

use utils::{read_config, write_config, User};

#[tauri::command]
fn greet(user: State<User>, _name: &str) -> String {
    // println!("{user:?}");
    serde_json::to_string(&user.inner()).unwrap()
}

#[tauri::command]
fn get_user(user: State<User>) -> String {
    serde_json::to_string(&user.inner()).unwrap()
}

#[tauri::command]
fn save_user(user_arc: State<User>, user: User) -> Result<(), String> {
    *user_arc.name.lock().unwrap() = user.name.lock().unwrap().clone();
    *user_arc.api_pass.lock().unwrap() = user.api_pass.lock().unwrap().clone();
    *user_arc.workspace.lock().unwrap() = user.workspace.lock().unwrap().clone();

    if write_config(user).is_ok() {
        return Ok(());
    }

    Err("Save user settings failed!".to_string())
}

fn main() {
    let user = read_config();

    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            let _ = win.move_window(Position::TopRight);

            Ok(())
        })
        .manage(user)
        .invoke_handler(tauri::generate_handler![greet, get_user, save_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
