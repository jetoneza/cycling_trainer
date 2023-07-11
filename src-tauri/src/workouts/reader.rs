use log::error;
use serde::Serialize;
use std::{env, fs, path::Path};

use super::zwo::{zwo_to_workout, WorkoutFile};

const LOGGER_NAME: &str = "workouts::reader";

#[derive(Serialize)]
pub struct WorkoutItem {
    pub id: usize,
    pub name: String,
    pub description: String
} 

pub fn get_workouts_from_file() -> Vec<WorkoutFile> {
    // TODO: Use event based reading for large XML files
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root_dir = match Path::new(manifest_dir).parent() {
        Some(dir) => dir,
        None => {
            error!(
                "{}:get_workouts: Unable to retrieve root directory.",
                LOGGER_NAME
            );

            return Vec::new();
        }
    };

    let path = root_dir.join("workouts");

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
