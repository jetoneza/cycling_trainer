#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct HeartRateMeasurement {
    pub bpm: u16,
}

pub fn parse_hrm_data(data: &Vec<u8>) -> HeartRateMeasurement {
    // TODO: Complete parsing of data
    // e.g. detection if hrm monitor is in contact of body
    HeartRateMeasurement {
        bpm: data[1] as u16,
    }
}
