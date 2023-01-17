#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_positioner::{Position, WindowExt};

pub mod utils;

use utils::{get_user, get_work, greet, read_config, read_user, read_work, save_user, save_work};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    let config = read_config();
    let user = read_user();
    let work = read_work();

    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            let _ = win.move_window(Position::TopRight);

            Ok(())
        })
        .manage(config)
        .manage(user)
        .manage(work)
        .invoke_handler(tauri::generate_handler![
            greet, get_user, get_work, save_user, save_work
        ])
        .system_tray(system_tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                let item_handle = app.tray_handle().get_item("hide");

                if window.is_visible().unwrap() {
                    item_handle.set_title("Hide").unwrap();
                } else {
                    item_handle.set_title("Show").unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let window = app.get_window("main").unwrap();
                let item_handle = app.tray_handle().get_item(&id);

                match id.as_str() {
                    "hide" => {
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        } else {
                            window.show().unwrap();
                            item_handle.set_title("Hide").unwrap();
                            let _ = window.move_window(Position::TopRight);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
