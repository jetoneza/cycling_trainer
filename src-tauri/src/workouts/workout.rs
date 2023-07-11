use log::warn;
use std::sync::OnceLock;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::{reader::get_workouts_from_file, zwo::WorkoutFile};

pub static WORKOUTS: OnceLock<RwLock<Vec<Workout>>> = OnceLock::new();

pub struct Workout {
    pub id: Uuid,

    // TODO: Convert deserialized data from other formats to a common structure
    pub workout_file: WorkoutFile,
}

pub fn init() {
    let files = get_workouts_from_file();
    let workouts: Vec<Workout> = files
        .iter()
        .map(|file| Workout {
            id: Uuid::new_v4(),
            workout_file: file.clone(),
        })
        .collect();

    if let Err(_) = WORKOUTS.set(RwLock::new(workouts)) {
        warn!("Unable to load workouts.");
        return;
    }
}
