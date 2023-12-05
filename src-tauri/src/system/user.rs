use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::{fs, sync::OnceLock};
use tokio::sync::RwLock;

use super::directory::get_user_settings_file;

pub static APP_USER: OnceLock<RwLock<User>> = OnceLock::new();

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub settings: UserSettings,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub ftp: u8,
}

pub fn load_app_user() {
    let result = get_user_settings_file();

    let Ok(path) = result else {
        warn!("Unable to load user settings path.");
        return;
    };

    let user = match fs::File::open(&path) {
        Ok(file) => match serde_json::from_reader::<_, User>(file) {
            Ok(user) => user,
            Err(err) => {
                error!("Error deserializing user settings file: {}", err);
                return;
            }
        },
        Err(err) => {
            error!("Error opening user settings file: {}", err);
            return;
        }
    };

    if let Err(_) = APP_USER.set(RwLock::new(user)) {
        warn!("Unable to load user settings.");
    }
}
