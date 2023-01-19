use tauri::State;

pub mod config;

pub use config::{read_settings, write_settings, Settings, User};

#[tauri::command]
pub fn get_settings(settings: State<Settings>) -> String {
    serde_json::to_string(&settings.inner().user).unwrap()
}

#[tauri::command]
pub fn save_settings(settings: State<Settings>, user: User) -> Result<String, String> {
    *settings.user.lock().unwrap() = user;

    if write_settings(&settings.inner()).is_ok() {
        return Ok("Save settings success!".to_string());
    }

    Err("Save config failed!".to_string())
}
