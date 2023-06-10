#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct HeartRateMeasurement {
    pub bpm: u16,
}

pub fn parse_hrm_data(data: &Vec<u8>) -> HeartRateMeasurement {
    HeartRateMeasurement {
        bpm: data[1] as u16,
    }
}
