use std::sync::Mutex;

use preferences::{AppInfo, Preferences};
use serde::{Deserialize, Serialize};

const APP_INFO: AppInfo = AppInfo {
    name: "Kimaier",
    author: "Jonathan Baecker",
};

const CONFIG_PATH: &str = "config/kimaier/kimaier";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    pub user: Mutex<User>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub api_pass: String,
    pub api_url: String,
    pub project: String,
    pub activity: String,
    pub project_id: i32,
    pub activity_id: i32,
}

pub fn read_settings() -> Settings {
    match Settings::load(&APP_INFO, CONFIG_PATH) {
        Ok(settings) => settings,
        Err(_) => Settings::default(),
    }
}

pub fn write_settings(settings: &Settings) -> Result<(), preferences::PreferencesError> {
    settings.save(&APP_INFO, CONFIG_PATH)
}
