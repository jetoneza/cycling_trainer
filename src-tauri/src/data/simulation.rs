use std::sync::OnceLock;

use tauri::Manager;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use crate::TAURI_APP_HANDLE;

use super::session::SessionStatus;
use super::{
    heart_rate_measurement::HeartRateMeasurement,
    indoor_bike_data::IndoorBikeData,
    session::{self, Session},
};

pub static SIMULATION: OnceLock<Simulation> = OnceLock::new();

pub struct Simulation {
    session: RwLock<Option<Session>>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            session: RwLock::new(None),
        }
    }

    pub async fn start(&self) {
        let mut session_guard = self.session.write().await;
        let mut session = Session::new();

        session.start_session();

        *session_guard = Some(session);

        tokio::spawn(handle_notifications());
    }

    pub async fn stop(&self) {
        let mut session_guard = self.session.write().await;
        let Some(session) = session_guard.as_mut() else {
            return;
        };

        session.stop_session();
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
        match (SIMULATION.get(), TAURI_APP_HANDLE.lock().await.as_ref()) {
            (Some(sim), Some(app)) => {
                let mut session_guard = sim.session.write().await;
                let Some(session) = session_guard.as_mut() else {
                    continue;
                };

                if let SessionStatus::Stopped = session.status {
                    break;
                }

                // TODO: Simulate values
                let hr_data = HeartRateMeasurement {
                    bpm: 85,
                    is_sensor_in_contact: true,
                    is_sensor_contact_supported: true,
                };

                let mut bike_data = IndoorBikeData {
                    cadence: Some(100),
                    speed: Some(30),
                    distance: Some(0),
                    power: Some(100),
                };

                session.add_heart_rate_data(hr_data.bpm);

                match (
                    bike_data.cadence,
                    bike_data.speed,
                    bike_data.power,
                    bike_data.distance,
                ) {
                    (Some(cadence), Some(speed), Some(power), _) => {
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

                app.emit_all("hrm_notification", hr_data).ok();
                app.emit_all("indoor_bike_notification", bike_data).ok();
            }
            _ => {}
        }

        sleep(Duration::from_millis(1000)).await;
    }
}
