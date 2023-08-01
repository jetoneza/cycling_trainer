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

    pub fn get_total_distance(&self) -> u32 {
        self.total_distance
    }

    pub fn set_total_distance(&mut self, distance: u32) {
        self.total_distance = distance;
    }
}
