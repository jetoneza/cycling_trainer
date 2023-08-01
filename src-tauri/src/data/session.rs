use std::time::{Duration, Instant};

const KPH_TO_KPS: f64 = 1.0 / 3600.0;

pub enum SessionStatus {
    Started,
    Paused,
    Stopped,
}

pub struct IndoorBikeData {
    pub cadence: u16,
    pub speed: u16,
    pub power: u16,
}

pub struct Session {
    pub status: SessionStatus,
    pub indoor_bike_data: Vec<IndoorBikeData>,
    pub heart_rate_data: Vec<u16>,
    pub total_distance: u32,
}

impl Session {
    pub fn new() -> Self {
        Self {
            status: SessionStatus::Stopped,
            indoor_bike_data: Vec::new(),
            heart_rate_data: Vec::new(),
            total_distance: 0,
        }
    }

    pub fn start_session(&mut self) {
        self.status = SessionStatus::Started;
    }

    pub fn pause_session(&mut self) {
        self.status = SessionStatus::Paused;
    }

    pub fn stop_session(&mut self) {
        self.status = SessionStatus::Stopped;
    }

    pub fn add_indoor_bike_data(&mut self, data: IndoorBikeData) {
        self.indoor_bike_data.push(data);
    }

    pub fn add_heart_rate_data(&mut self, bpm: u16) {
        self.heart_rate_data.push(bpm);
    }

    pub fn calculate_total_distance(&mut self, speed: u16) -> u32 {
        let elapsed_time = get_time_change();

        let speed_kps = (speed as f64) * KPH_TO_KPS;
        let distance = ((speed_kps * elapsed_time.as_secs_f64()) * 1000.0) as u32;

        self.total_distance += distance;

        self.total_distance
    }
}

fn get_time_change() -> Duration {
    static mut LAST_CALL: Option<Instant> = None;

    let now = Instant::now();

    let last_time = unsafe { LAST_CALL.replace(now).unwrap_or(now) };

    now - last_time
}
