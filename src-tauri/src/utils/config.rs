use std::sync::Mutex;

use preferences::{AppInfo, Preferences};
use serde::{Deserialize, Serialize};

const APP_INFO: AppInfo = AppInfo {
    name: "Kimaier",
    author: "Jonathan Baecker <jonbae77@gmail.com>",
};

const CONFIG_PATH: &str = "config/kimaier";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: Mutex<String>,
    pub api_pass: Mutex<String>,
    pub api_url: Mutex<String>,
    pub workspace: Mutex<String>,
}

impl User {
    pub fn new(name: String, api_pass: String, api_url: String, workspace: String) -> Self {
        Self {
            name: Mutex::new(name),
            api_pass: Mutex::new(api_pass),
            api_url: Mutex::new(api_url),
            workspace: Mutex::new(workspace),
        }
    }

    pub fn empty() -> Self {
        Self {
            name: Mutex::new(String::new()),
            api_pass: Mutex::new(String::new()),
            api_url: Mutex::new(String::new()),
            workspace: Mutex::new(String::new()),
        }
    }
}

pub fn read_config() -> User {
    match User::load(&APP_INFO, CONFIG_PATH) {
        Ok(user) => user,
        Err(_) => User::empty(),
    }
}

pub fn write_config(user: User) -> Result<(), preferences::PreferencesError> {
    user.save(&APP_INFO, CONFIG_PATH)
}
