#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct IndoorBikeData {
    pub cadence: u16,
    pub speed: u16,
}

// Resource:
// https://www.bluetooth.com/specifications/specs/gatt-specification-supplement/
// Check for Indoor Bike Data
pub fn parse_indoor_bike_data(data: &Vec<u8>) -> IndoorBikeData {
    let speed = get_speed(data);

    IndoorBikeData { cadence: 86, speed }
}

// Instantaneous Speed
// Data type: u16
// Size (octets): 0 or 2
fn get_speed(data: &Vec<u8>) -> u16 {
    let flags = get_flags(data);

    // Present if bit 0 of Flags field is set to 0
    let is_speed_present = flags & 1 == 0;

    if !is_speed_present {
        return 0;
    }

    let raw_speed = combine_u8_to_u16(data[2], data[3]);

    // Unit is 1/100 of a kilometer per hour
    raw_speed / 100
}

// Flags field
// 0 - More data
fn get_flags(data: &Vec<u8>) -> u16 {
    combine_u8_to_u16(data[0], data[1])
}

fn combine_u8_to_u16(first: u8, second: u8) -> u16 {
    (first as u16) | (second as u16) << 8
}
