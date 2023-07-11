use std::{fs, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct WorkoutFile {
    pub name: String,
    pub description: String,
    pub workout: Workout,

    #[serde(rename = "sportType")]
    sport_type: String,
    author: String,
    tags: Tags,
}

#[derive(Deserialize, Clone)]
struct Tags {
    tag: Vec<Tag>,
}

#[derive(Deserialize, Clone)]
struct Tag {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Deserialize, Clone)]
pub struct Workout {
    #[serde(rename = "$value")]
    pub workouts: Vec<WorkoutType>,
}

#[derive(Deserialize, Clone)]
pub enum WorkoutType {
    Warmup {
        #[serde(rename = "@Duration")]
        duration: u16,
        #[serde(rename = "@PowerLow")]
        power_low: f64,
        #[serde(rename = "@PowerHigh")]
        power_high: f64,
        #[serde(rename = "@Cadence")]
        cadence: u8,
    },
    SteadyState {
        #[serde(rename = "@Duration")]
        duration: u16,
        #[serde(rename = "@Power")]
        power: f64,
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
        #[serde(rename = "@Cadence")]
        cadence: u8,
    },
}

pub fn zwo_to_workout(file_path: &PathBuf) -> Option<WorkoutFile> {
    let xml = fs::read_to_string(file_path).ok()?;

    quick_xml::de::from_str(xml.as_str()).ok()
}
