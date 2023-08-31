use std::sync::OnceLock;

use tauri::Manager;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{sleep, Duration};

use crate::TAURI_APP_HANDLE;

use super::session::SessionStatus;
use super::{
    heart_rate_measurement::HeartRateMeasurement,
    indoor_bike_data::IndoorBikeData,
    session::{self, Session},
};

pub static SIMULATION: OnceLock<Simulation> = OnceLock::new();

pub enum SimulationStatus {
    Started,
    Paused,
    Stopped,
}

pub struct Simulation {
    status: Mutex<SimulationStatus>,
    session: RwLock<Option<Session>>,
}

impl Simulation {
    pub fn new() -> Self {
        tokio::spawn(handle_notifications());

        Self {
            status: Mutex::new(SimulationStatus::Paused),
            session: RwLock::new(None),
        }
    }

    pub async fn start(&self) {
        *self.status.lock().await = SimulationStatus::Started;
    }

    pub async fn stop(&self, action: &str) {
        let status = match action {
            "stop" => SimulationStatus::Stopped,
            "pause" => SimulationStatus::Paused,
            _ => SimulationStatus::Started,
        };

        *self.status.lock().await = status;
    }

    pub async fn start_session(&self) {
        let mut session_guard = self.session.write().await;
        let mut session = Session::new();

        session.start_session();

        *session_guard = Some(session);
    }

    pub async fn stop_session(&self, action: &str) {
        let mut session_guard = self.session.write().await;
        let Some(session) = session_guard.as_mut() else {
            return;
        };

        match action {
            "stop" => session.stop_session(),
            "pause" => session.pause_session(),
            _ => {}
        }
    }

    pub async fn get_session_data(&self) -> Session {
        let session_guard = self.session.read().await;
        let Some(session) = session_guard.as_ref() else {
            return Session::new();
        };

        session.get_session_data()
    }
}

async fn handle_notifications() {
    loop {
        let Some(sim) = SIMULATION.get() else {
            break;
        };

        let app_guard = TAURI_APP_HANDLE.lock().await;
        let Some(app) = app_guard.as_ref() else {
            break;
        };

        let mut bpm = 0;
        let mut cadence = None;
        let mut speed = None;
        let mut power = None;

        if let SimulationStatus::Started = *sim.status.lock().await {
            // TODO: Calculate simulated values
            bpm = 100;
            cadence = Some(100);
            speed = Some(30);
            power = Some(120);
        }

        if let SimulationStatus::Stopped = *sim.status.lock().await {
            break;
        }

        let hr_data = HeartRateMeasurement {
            bpm,
            is_sensor_in_contact: true,
            is_sensor_contact_supported: true,
        };

        let mut bike_data = IndoorBikeData {
            cadence,
            speed,
            distance: None,
            power,
        };

        let mut session_guard = sim.session.write().await;
        if let Some(session) = session_guard.as_mut() {
            if let SessionStatus::Started = session.status {
                session.add_heart_rate_data(hr_data.bpm);

                match (bike_data.cadence, bike_data.speed, bike_data.power) {
                    (Some(cadence), Some(speed), Some(power)) => {
                        session.add_indoor_bike_data(session::IndoorBikeData {
                            cadence,
                            speed,
                            power,
                        });
                    }
                    _ => {}
                }

                match (bike_data.speed, bike_data.distance) {
                    (Some(speed), None) => {
                        let distance = session.calculate_total_distance(speed);

                        bike_data.distance = Some(distance);
                    }
                    (_, Some(distance)) => {
                        session.set_total_distance(distance);
                    }
                    _ => {}
                }
            }
        }

        app.emit_all("hrm_notification", hr_data.to_owned()).ok();
        app.emit_all("indoor_bike_notification", bike_data.to_owned())
            .ok();

        sleep(Duration::from_millis(1000)).await;
    }
}
