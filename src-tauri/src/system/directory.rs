use std::fs;

use log::{error, info};

const LOGGER_NAME: &str = "system::directory";

/// Initializes app startup requirements
///
/// - Checks directories
/// - Checks user settings
pub fn initialize() {
    match dirs::document_dir() {
        Some(dir) => {
            let app_folder = dir.join("Cycling Trainer");

            if !app_folder.exists() {
                info!("{}: Creating Cycling Trainer folder...", LOGGER_NAME);

                if let Err(err) = fs::create_dir(&app_folder) {
                    error!(
                        "{}:get_workouts: Unable to create directory. {}",
                        LOGGER_NAME, err
                    );
                }
            }

            let workouts_dir = app_folder.join("workouts");

            if !workouts_dir.exists() {
                info!("{}: Creating workouts folder...", LOGGER_NAME);

                if let Err(err) = fs::create_dir(&workouts_dir) {
                    error!(
                        "{}:get_workouts: Unable to create directory. {}",
                        LOGGER_NAME, err
                    );
                }
            }
        }
        None => {
            error!(
                "{}:get_workouts: Unable to retrieve root directory.",
                LOGGER_NAME
            );
        }
    };
}
