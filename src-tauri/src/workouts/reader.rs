use log::{error, info};
use serde::Serialize;
use std::fs;

use super::zwo::{zwo_to_workout, WorkoutFile};

const LOGGER_NAME: &str = "workouts::reader";

#[derive(Serialize)]
pub struct WorkoutItem {
    pub id: usize,
    pub name: String,
    pub description: String,
}

pub fn get_workouts_from_file() -> Vec<WorkoutFile> {
    // TODO: Use event based reading for large XML files
    let path = match dirs::document_dir() {
        Some(dir) => {
            let app_folder = dir.join("Cycling Trainer");

            if !app_folder.exists() {
                info!("{}: Creating Cycling Trainer folder...", LOGGER_NAME);

                if let Err(err) = fs::create_dir(&app_folder) {
                    error!(
                        "{}:get_workouts: Unable to create directory. {}",
                        LOGGER_NAME, err
                    );

                    return Vec::new();
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

                    return Vec::new();
                }
            }

            workouts_dir
        }
        None => {
            error!(
                "{}:get_workouts: Unable to retrieve root directory.",
                LOGGER_NAME
            );

            return Vec::new();
        }
    };

    let workout_files: Vec<WorkoutFile> = match fs::read_dir(path) {
        Ok(files) => files
            .filter_map(|entry| {
                let entry = entry.ok()?;

                let file_path = entry.path();
                let extension = file_path.extension()?;

                match extension.to_string_lossy().to_string().as_str() {
                    "zwo" => zwo_to_workout(&file_path),

                    // TODO: Support other file times
                    _ => {
                        error!("{}:get_workouts: Unsupported file type", LOGGER_NAME);
                        None
                    }
                }
            })
            .collect(),
        Err(error) => {
            error!(
                "{}:get_workouts: Error while reading directory: {:?}",
                LOGGER_NAME, error
            );

            Vec::new()
        }
    };

    workout_files
}
