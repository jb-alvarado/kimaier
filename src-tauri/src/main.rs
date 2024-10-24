#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use tauri::Manager;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    WindowEvent, Wry,
};
use tauri_plugin_positioner::{Position, WindowExt};
// use tauri_plugin_window_state::StateFlags;

struct AppState {
    hide_menu_item: Mutex<Option<MenuItem<Wry>>>,
}

fn main() {
    // prevent "Error 71 dispatching to Wayland display." error
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        // .plugin(
        //     tauri_plugin_window_state::Builder::default()
        //         .with_state_flags(StateFlags::POSITION)
        //         .build(),
        // )
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            hide_menu_item: Mutex::new(None),
        })
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&hide, &quit])?;
            let window = app.get_webview_window("main").unwrap();
            let _ = window.as_ref().window().move_window(Position::TopRight);
            let window_clone = window.clone();
            let window_clone2 = window.clone();

            let app_handle = app.handle();
            {
                let app_state = app_handle.state::<AppState>();
                *app_state.hide_menu_item.lock().unwrap() = Some(hide.clone());
            }

            let _ = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "hide" => {
                        if window_clone.is_visible().unwrap_or_default() {
                            window_clone.hide().expect("hide window");
                            hide.set_text("Show").expect("set text");
                        } else {
                            window_clone.show().expect("show window");
                            hide.set_text("Hide").expect("set text");
                        };
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .on_tray_icon_event(move |_, event| match event {
                    TrayIconEvent::DoubleClick {
                        position: _,
                        rect: _,
                        ..
                    } => {
                        println!("DoubleClick");
                        let app_state = window_clone2.state::<AppState>();
                        let hide_menu_item = app_state.hide_menu_item.lock().unwrap();

                        if window_clone2.is_visible().unwrap_or_default() {
                            if let Some(hide) = hide_menu_item.as_ref() {
                                hide.set_text("Hide").expect("set text");
                            }
                        } else {
                            window_clone2.show().unwrap();

                            println!("window not visible");

                            if let Some(hide) = hide_menu_item.as_ref() {
                                hide.set_text("Show").expect("set text");
                            }
                        }
                    }

                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                let app_state = window.state::<AppState>();
                let hide_menu_item = app_state.hide_menu_item.lock().unwrap();
                if let Some(hide) = hide_menu_item.as_ref() {
                    hide.set_text("Show").expect("set text");
                }
                window.hide().expect("hide window");
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
