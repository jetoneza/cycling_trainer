use log::error;
use quick_xml;
use serde::Deserialize;
use std::{env, fs, path::Path};

const LOGGER_NAME: &str = "workouts::reader";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WorkoutFile {
    #[serde(rename = "sportType")]
    sport_type: String,
    author: String,
    name: String,
    description: String,
    tags: Tags,
    workout: Workout,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Tags {
    tag: Vec<Tag>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Tag {
    #[serde(rename = "@name")]
    name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Workout {
    #[serde(rename = "$value")]
    workouts: Vec<WorkoutType>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
enum WorkoutType {
    Warmup {
        #[serde(rename = "@Duration")]
        duration: u16,
        #[serde(rename = "@PowerLow")]
        power_low: f64,
        #[serde(rename = "@PowerHigh")]
        power_high: f64,
        #[serde(rename = "@pace")]
        pace: u32,
        #[serde(rename = "@Cadence")]
        cadence: u8,
    },
    SteadyState {
        #[serde(rename = "@Duration")]
        duration: u16,
        #[serde(rename = "@Power")]
        power: f64,
        #[serde(rename = "@pace")]
        pace: u32,
        #[serde(rename = "@Cadence")]
        cadence: u8,
    },
    Cooldown {
        #[serde(rename = "@Duration")]
        duration: u16,
        #[serde(rename = "@PowerLow")]
        power_low: f64,
        #[serde(rename = "@PowerHigh")]
        power_high: f64,
        #[serde(rename = "@pace")]
        pace: u32,
        #[serde(rename = "@Cadence")]
        cadence: u8,
    },
}

pub fn get_workouts() -> Vec<WorkoutFile> {
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
                    "zwo" => {
                        let xml = fs::read_to_string(file_path).ok()?;

                        quick_xml::de::from_str(xml.as_str()).ok()
                    }

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
