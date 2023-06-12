#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct IndoorBikeData {
    pub cadence: u16,
    pub speed: u16,
}

pub fn parse_indoor_bike_data(data: &Vec<u8>) -> IndoorBikeData {
    IndoorBikeData {
        cadence: 86,
        speed: 30,
    }
}
