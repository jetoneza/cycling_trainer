use log::warn;
use serde::Serialize;
use std::sync::OnceLock;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::{
    reader::get_workouts_from_file,
    zwo::{self, WorkoutFile},
};

pub static ACTIVITIES: OnceLock<RwLock<Vec<Activity>>> = OnceLock::new();

#[derive(Serialize, Clone)]
pub struct Activity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub workouts: Vec<Workout>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Workout {
    pub workout_type: WorkoutType,
    pub duration: u16,
    pub cadence: u8,
    pub power_low: f64,
    pub power_high: f64,
    pub power_steady: f64,
}

#[derive(Serialize, Clone)]
pub enum WorkoutType {
    Warmup,
    SteadyState,
    Cooldown,
}

impl From<WorkoutFile> for Activity {
    fn from(value: WorkoutFile) -> Self {
        let workouts = value
            .workout
            .workouts
            .iter()
            .map(|workout| {
                let workout_type = match workout {
                    zwo::WorkoutType::Warmup { .. } => WorkoutType::Warmup,
                    zwo::WorkoutType::SteadyState { .. } => WorkoutType::SteadyState,
                    zwo::WorkoutType::Cooldown { .. } => WorkoutType::Cooldown,
                };

                let (duration, cadence, power_high, power_low, power_steady) = match workout {
                    zwo::WorkoutType::Warmup {
                        duration,
                        power_low,
                        power_high,
                        cadence,
                    }
                    | zwo::WorkoutType::Cooldown {
                        duration,
                        power_low,
                        power_high,
                        cadence,
                    } => (
                        duration.to_owned(),
                        cadence.to_owned(),
                        power_high.to_owned(),
                        power_low.to_owned(),
                        0.0,
                    ),
                    zwo::WorkoutType::SteadyState {
                        duration,
                        power,
                        cadence,
                    } => (
                        duration.to_owned(),
                        cadence.to_owned(),
                        0.0,
                        0.0,
                        power.to_owned(),
                    ),
                };

                Workout {
                    workout_type,
                    cadence,
                    duration,
                    power_high,
                    power_low,
                    power_steady,
                }
            })
            .collect();

        Activity {
            id: Uuid::new_v4().to_string(),
            name: value.name,
            description: value.description,
            workouts,
        }
    }
}

pub fn load_activities() {
    let files = get_workouts_from_file();
    let activities: Vec<Activity> = files
        .iter()
        .map(|file| Activity::from(file.to_owned()))
        .collect();

    if let Err(_) = ACTIVITIES.set(RwLock::new(activities)) {
        warn!("Unable to load workouts.");
        return;
    }
}
