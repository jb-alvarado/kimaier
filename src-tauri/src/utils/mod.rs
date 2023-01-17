use tauri::State;

pub mod config;

pub use config::{read_config, read_user, read_work, write_config, Config, User, Work};

#[tauri::command]
pub fn greet(user: State<User>, _name: &str) -> String {
    // println!("{user:?}");
    serde_json::to_string(&user.inner()).unwrap()
}

#[tauri::command]
pub fn get_config(config: State<Config>) -> String {
    println!("{config:?}");
    serde_json::to_string(&config.inner()).unwrap()
}

#[tauri::command]
pub fn get_user(user: State<User>) -> String {
    serde_json::to_string(&user.inner()).unwrap()
}

#[tauri::command]
pub fn get_work(work: State<Work>) -> String {
    serde_json::to_string(&work.inner()).unwrap()
}

#[tauri::command]
pub fn save_user(config: State<Config>, user: User) -> Result<String, String> {
    *config.user.name.lock().unwrap() = user.name.lock().unwrap().clone();
    *config.user.api_pass.lock().unwrap() = user.api_pass.lock().unwrap().clone();
    *config.user.api_url.lock().unwrap() = user.api_url.lock().unwrap().clone();

    if write_config(&config.inner()).is_ok() {
        return Ok("Save config success!".to_string());
    }

    Err("Save config failed!".to_string())
}

#[tauri::command]
pub fn save_work(config: State<Config>, work: Work) -> Result<String, String> {
    *config.work.project.lock().unwrap() = work.project.lock().unwrap().clone();
    *config.work.activity.lock().unwrap() = work.activity.lock().unwrap().clone();
    *config.work.project_id.lock().unwrap() = work.project_id.lock().unwrap().clone();
    *config.work.activity_id.lock().unwrap() = work.activity_id.lock().unwrap().clone();

    if write_config(&config.inner()).is_ok() {
        return Ok("Save work success!".to_string());
    }

    Err("Save work failed!".to_string())
}
