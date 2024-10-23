#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&hide, &quit])?;
            let win = app.get_webview_window("main").unwrap();
            let _ = win.move_window(Position::TopRight);
            let win_clone = win.clone();
            let _ = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "hide" => {
                        if win_clone.is_visible().unwrap_or_default() {
                            win_clone.hide().expect("hide window");
                            hide.set_text("Show").expect("set text");
                        } else {
                            win_clone.show().expect("show window");
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
                .build(app)?;

            Ok(())
        })
        // .plugin(tauri_plugin_window_state::Builder::default().build())
        // .on_window_event(|window, event| {
        //     println!("event {:?}", window.is_visible());
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
