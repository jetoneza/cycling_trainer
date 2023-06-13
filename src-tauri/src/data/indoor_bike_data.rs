#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct IndoorBikeData {
    pub cadence: u16,
    pub speed: u16,
}

enum IndoorBikeDataType {
    Speed,
    Cadence,
}

// Size in octets
const FLAGS_SIZE: usize = 2;
const AVERAGE_SPEED_SIZE: usize = 2;
const INSTANTANEOUS_SPEED_SIZE: usize = 2;

// Resource:
// https://www.bluetooth.com/specifications/specs/gatt-specification-supplement/
// Check for Indoor Bike Data
pub fn parse_indoor_bike_data(data: &Vec<u8>) -> IndoorBikeData {
    let speed = get_speed(data);
    let cadence = get_cadence(data);

    IndoorBikeData { cadence, speed }
}

// Instantaneous Speed
// Data type: u16
// Size (octets): 0 or 2
fn get_speed(data: &Vec<u8>) -> u16 {
    // Present if bit 0 of Flags field is set to 0
    if !is_speed_present(data) {
        return 0;
    }

    let Some(data_index) = get_data_index(data, IndoorBikeDataType::Speed) else {
        return 0;
    };

    let raw_speed = combine_u8_to_u16(data[data_index.0], data[data_index.1]);

    // Unit is 1/100 of a kilometer per hour
    raw_speed / 100
}

// Instantaneous Cadence
// Data type: u16
// Size (octets): 0 or 2
fn get_cadence(data: &Vec<u8>) -> u16 {
    // Present if bit 2 of Flags field is set to 1
    if !is_cadence_present(data) {
        return 0;
    }

    let Some(data_index) = get_data_index(data, IndoorBikeDataType::Cadence) else {
        return 0;
    };

    let cadence = combine_u8_to_u16(data[data_index.0], data[data_index.1]);

    // Unit is 1/2 of a revolution per minute
    cadence / 2
}

fn get_data_index(data: &Vec<u8>, data_type: IndoorBikeDataType) -> Option<(usize, usize)> {
    match data_type {
        IndoorBikeDataType::Speed => Some((FLAGS_SIZE, FLAGS_SIZE + 1)),
        IndoorBikeDataType::Cadence => {
            let mut index = FLAGS_SIZE;

            if is_speed_present(data) {
                index += INSTANTANEOUS_SPEED_SIZE;
            }

            if is_ave_speed_present(data) {
                index += AVERAGE_SPEED_SIZE;
            }

            Some((index, index + 1))
        }
        _ => None,
    }
}

fn is_speed_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 1 == 0
}

fn is_ave_speed_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b10 == 0b10
}

fn is_cadence_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b100 == 0b100
}

// Flags field
// 0 - More data
// 1 - Average speed present
// 2 - Instantaneous cadence present
fn get_flags(data: &Vec<u8>) -> u16 {
    combine_u8_to_u16(data[0], data[1])
}

fn combine_u8_to_u16(first: u8, second: u8) -> u16 {
    (first as u16) | (second as u16) << 8
}