use std::{fs, path::PathBuf};

use crate::{error::error_generic, prelude::*};

use log::{error, info};

use super::user::{User, UserSettings};

const LOGGER_NAME: &str = "system::directory";

/// Initializes app startup requirements
///
/// - Checks directories
/// - Checks user settings
pub fn initialize() {
    match dirs::document_dir() {
        Some(dir) => {
            // App folder
            let app_dir = get_or_create_directory("Cycling Trainer", &dir);
            let Ok(app_folder) = app_dir else {
                return;
            };

            // Workouts directory
            let _ = get_or_create_directory("workouts", &app_folder);

            // User settings
            let _ = get_or_create_file("user_settings.json", &app_folder);
        }
        None => {
            error!(
                "{}:initialize: Unable to retrieve root directory.",
                LOGGER_NAME
            );
        }
    };
}

pub fn get_user_settings_file() -> Result<PathBuf> {
    match dirs::document_dir() {
        Some(dir) => {
            // App folder
            let app_dir = get_or_create_directory("Cycling Trainer", &dir);
            let Ok(app_folder) = app_dir else {
                return Err(error_generic("Unable retrieve app directory."));
            };

            // User settings
            get_or_create_file("user_settings.json", &app_folder)
        }
        None => {
            error!(
                "{}:initialize: Unable to retrieve root directory.",
                LOGGER_NAME
            );

            Err(error_generic("Unable retrieve root directory."))
        }
    }
}

fn get_or_create_directory(folder_name: &str, dir: &PathBuf) -> Result<PathBuf> {
    let folder = dir.join(folder_name);

    if !folder.exists() {
        info!("{}: Creating {} directory.", folder_name, LOGGER_NAME);

        if let Err(err) = fs::create_dir(&folder) {
            error!(
                "{}:get_or_create_directory: Unable to create directory. {}",
                LOGGER_NAME, err
            );

            return Err(error_generic("Unable to create directory"));
        }
    }

    Ok(folder)
}

fn get_or_create_file(filename: &str, dir: &PathBuf) -> Result<PathBuf> {
    let file = dir.join(filename);

    if !file.exists() {
        let default_user = User {
            username: "".to_string(),
            settings: UserSettings { ftp: 0 },
        };

        if let Err(err) = serde_json::to_writer_pretty(
            fs::File::create(&file).expect("Error creating user settings file."),
            &default_user,
        ) {
            error!(
                "{}:get_or_create_file: Error writing user settings file: {}",
                LOGGER_NAME, err
            );

            return Err(error_generic("Error writing user settings file"));
        }
    }

    Ok(file)
}
