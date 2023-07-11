use std::sync::OnceLock;
use tokio::sync::RwLock;
use log::warn;

use super::{reader::get_workouts_from_file, zwo::WorkoutFile};

pub static WORKOUTS: OnceLock<RwLock<Vec<WorkoutFile>>> = OnceLock::new();

pub fn init() {
    let workouts = get_workouts_from_file();

    if let Err(_) = WORKOUTS.set(RwLock::new(workouts)) {
        warn!("Unable to load workouts.");
        return;
    }
}
