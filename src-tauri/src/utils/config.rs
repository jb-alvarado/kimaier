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
}

impl User {
    pub fn new(name: String, api_pass: String, api_url: String) -> Self {
        Self {
            name: Mutex::new(name),
            api_pass: Mutex::new(api_pass),
            api_url: Mutex::new(api_url),
        }
    }

    pub fn empty() -> Self {
        Self {
            name: Mutex::new(String::new()),
            api_pass: Mutex::new(String::new()),
            api_url: Mutex::new(String::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    pub project: Mutex<String>,
    pub activity: Mutex<String>,
    pub project_id: Mutex<i32>,
    pub activity_id: Mutex<i32>,
}

impl Work {
    pub fn new(project: String, activity: String, project_id: i32, activity_id: i32) -> Self {
        Self {
            project: Mutex::new(project),
            activity: Mutex::new(activity),
            project_id: Mutex::new(project_id),
            activity_id: Mutex::new(activity_id),
        }
    }

    pub fn empty() -> Self {
        Self {
            project: Mutex::new(String::new()),
            activity: Mutex::new(String::new()),
            project_id: Mutex::new(0),
            activity_id: Mutex::new(0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user: User,
    pub work: Work,
}

impl Config {
    pub fn empty() -> Self {
        Self {
            user: User::empty(),
            work: Work::empty(),
        }
    }
}

pub fn read_config() -> Config {
    match Config::load(&APP_INFO, CONFIG_PATH) {
        Ok(config) => config,
        Err(_) => Config::empty(),
    }
}

pub fn read_user() -> User {
    match Config::load(&APP_INFO, CONFIG_PATH) {
        Ok(config) => config.user,
        Err(_) => User::empty(),
    }
}

pub fn read_work() -> Work {
    match Config::load(&APP_INFO, CONFIG_PATH) {
        Ok(config) => config.work,
        Err(_) => Work::empty(),
    }
}

pub fn write_config(config: &Config) -> Result<(), preferences::PreferencesError> {
    config.save(&APP_INFO, CONFIG_PATH)
}
