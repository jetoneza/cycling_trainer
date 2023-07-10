use std::{env, fs, path::Path};

use quick_xml;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WorkoutFile {
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

pub fn get_workouts() {
    // TODO: Use event based reading for large XML files
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root_dir = Path::new(manifest_dir)
        .parent()
        .expect("Unable to retrieve root directory");

    let path = root_dir.join("workouts").join("sample.zwo");

    println!("Path: {:?}", path);

    let xml = match fs::read_to_string(path) {
        Ok(result) => result,
        Err(error) => {
            println!("Error: {:?}", error);
            
            // TODO: Return an error instead
            "".to_string()
        }
    };

    let workout: WorkoutFile = quick_xml::de::from_str(xml.as_str()).unwrap();

    println!("{:?}", workout)
}
