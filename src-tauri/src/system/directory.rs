use std::{fs, path::PathBuf};

use crate::{error::error_generic, prelude::*};
use log::{error, info};

const LOGGER_NAME: &str = "system::directory";

/// Initializes app startup requirements
///
/// - Checks directories
/// - Checks user settings
pub fn initialize() {
    match dirs::document_dir() {
        Some(dir) => {
            // App folder
            let app_dir = get_or_create_directory("Cycling Trainer", dir);
            let Ok(app_folder) = app_dir else {
                return;
            };

            // Workouts directory
            let _ = get_or_create_directory("workouts", app_folder);
        }
        None => {
            error!(
                "{}:get_workouts: Unable to retrieve root directory.",
                LOGGER_NAME
            );
        }
    };
}

fn get_or_create_directory(folder_name: &str, dir: PathBuf) -> Result<PathBuf> {
    let folder = dir.join(folder_name);

    if !folder.exists() {
        info!("{}: Creating {} directory.", folder_name, LOGGER_NAME);

        if let Err(err) = fs::create_dir(&folder) {
            error!(
                "{}:get_workouts: Unable to create directory. {}",
                LOGGER_NAME, err
            );

            return Err(error_generic("Unable to create directory"));
        }
    }

    Ok(folder)
}
