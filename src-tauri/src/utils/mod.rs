use std::{
    error::Error
};

use reqwest::header;
use serde_json::json;
use tauri::State;

pub mod config;

pub use config::{read_config, write_config, User};

#[tauri::command]
pub fn greet(user: State<User>, _name: &str) -> String {
    // println!("{user:?}");
    serde_json::to_string(&user.inner()).unwrap()
}

#[tauri::command]
pub fn get_user(user: State<User>) -> String {
    serde_json::to_string(&user.inner()).unwrap()
}

#[tauri::command]
pub fn save_user(user_arc: State<User>, user: User) -> Result<String, String> {
    *user_arc.name.lock().unwrap() = user.name.lock().unwrap().clone();
    *user_arc.api_pass.lock().unwrap() = user.api_pass.lock().unwrap().clone();
    *user_arc.api_url.lock().unwrap() = user.api_url.lock().unwrap().clone();
    *user_arc.workspace.lock().unwrap() = user.workspace.lock().unwrap().clone();

    if write_config(user).is_ok() {
        return Ok("Save settings success!".to_string());
    }

    Err("Save user settings failed!".to_string())
}

fn get_request(user: &User, direction: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = format!("{}/{direction}", *user.api_url.lock().unwrap());
    let user_name = user.name.lock().unwrap().clone();
    let api_pass = user.api_pass.lock().unwrap().clone();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-AUTH-USER", header::HeaderValue::from_str(&user_name).unwrap());
    headers.insert("X-AUTH-TOKEN", header::HeaderValue::from_str(&api_pass).unwrap());

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client
    .get(&url)
    .send();

    let result = response?.json::<serde_json::Value>()?;

    Ok(result)
}

#[tauri::command]
pub fn get_activities(user: State<User>) -> String {
    if let Ok(activities) = get_request(user.inner(), "api/timesheets/active") {
        return serde_json::to_string(&activities).unwrap();
    }

    serde_json::to_string(&json!([])).unwrap()
}

#[tauri::command]
pub fn start_activity(user: State<User>) -> String {

    if let Ok(activities) = get_request(user.inner(), "api/timesheets/active") {
        return serde_json::to_string(&activities).unwrap();
    }

    serde_json::to_string(&json!([])).unwrap()
}
