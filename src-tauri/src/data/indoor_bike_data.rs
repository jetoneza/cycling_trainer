use std::time::{Duration, Instant};

use crate::utils::byte::{combine_u8_to_u16, combine_u8_to_u32};

#[derive(PartialEq, Clone, serde::Serialize)]
pub struct IndoorBikeData {
    pub cadence: Option<u16>,
    pub speed: Option<u16>,
    pub distance: Option<u32>,
    pub power: Option<u16>,
}

enum IndoorBikeDataType {
    Speed,
    Cadence,
    Distance,
    Power,
}

const KPH_TO_KPS: f64 = 1.0 / 3600.0;

// Size in octets
const FLAGS_SIZE: usize = 2;
const INSTANTANEOUS_SPEED_SIZE: usize = 2;
const AVERAGE_SPEED_SIZE: usize = 2;
const INSTANTANEOUS_CADENCE_SIZE: usize = 2;
const AVERAGE_CADENCE_SIZE: usize = 2;
const TOTAL_DISTANCE_SIZE: usize = 3;
const RESISTANCE_LEVEL_SIZE: usize = 1;

// Resource:
// https://www.bluetooth.com/specifications/specs/gatt-specification-supplement/
// Check for Indoor Bike Data
pub fn parse_indoor_bike_data(data: &Vec<u8>) -> IndoorBikeData {
    let elapsed_time = get_time_change();

    let speed = get_speed(data);
    let cadence = get_cadence(data);
    let distance = get_distance(data, speed, elapsed_time);
    let power = get_power(data);

    IndoorBikeData {
        cadence,
        speed,
        distance,
        power,
    }
}

// Instantaneous Speed
// Data type: u16
// Size (octets): 0 or 2
fn get_speed(data: &Vec<u8>) -> Option<u16> {
    // Present if bit 0 of Flags field is set to 0
    if !is_speed_present(data) {
        return None;
    }

    let data_index = get_data_index(data, IndoorBikeDataType::Speed);

    let raw_speed = combine_u8_to_u16(data[data_index], data[data_index + 1]);

    // Unit is 1/100 of a kilometer per hour
    Some(raw_speed / 100)
}

// Instantaneous Cadence
// Data type: u16
// Size (octets): 0 or 2
fn get_cadence(data: &Vec<u8>) -> Option<u16> {
    // Present if bit 2 of Flags field is set to 1
    if !is_cadence_present(data) {
        return None;
    }

    let data_index = get_data_index(data, IndoorBikeDataType::Cadence);

    let cadence = combine_u8_to_u16(data[data_index], data[data_index + 1]);

    // Unit is 1/2 of a revolution per minute
    Some(cadence / 2)
}

// Total Distance since the beginning of the training session
// Data type: u24
// Size (octets): 0 or 3
fn get_distance(data: &Vec<u8>, speed: Option<u16>, elapsed_time: Duration) -> Option<u32> {
    // TODO: Only calculate distance if the session has started
    static mut TOTAL_DISTANCE: Option<u32> = None;

    // Present if bit 4 of Flags field is set to 1
    if !is_distance_present(data) {
        // TODO: Calculate speed with other parameters, e.g. (Wind, Drafting, etc.)
        let distance = get_distance_from_speed(speed, elapsed_time)?;

        let total_distance = unsafe {
            TOTAL_DISTANCE = TOTAL_DISTANCE
                .map(|total| total + distance)
                .or(Some(distance));

            TOTAL_DISTANCE
        };

        // Calculated distance from speed in meters
        return total_distance;
    }

    let data_index = get_data_index(data, IndoorBikeDataType::Distance);

    let distance = combine_u8_to_u32(data[data_index], data[data_index + 1], data[data_index + 2]);

    // Distance is in meters
    Some(distance)
}

// Instantaneous Power
// Data type: i16
// Size (octets): 0 or 2
fn get_power(data: &Vec<u8>) -> Option<u16> {
    // Present if bit 6 of Flags field is set to 1
    if !is_power_present(data) {
        return None;
    }

    let data_index = get_data_index(data, IndoorBikeDataType::Power);

    let power = combine_u8_to_u16(data[data_index], data[data_index + 1]);

    // Watts
    Some(power)
}

fn get_distance_from_speed(speed: Option<u16>, elapsed_time: Duration) -> Option<u32> {
    let speed_kps = (speed? as f64) * KPH_TO_KPS;
    let distance = (speed_kps * elapsed_time.as_secs_f64()) * 1000.0;

    // Distance is in meters
    Some(distance as u32)
}

fn get_data_index(data: &Vec<u8>, data_type: IndoorBikeDataType) -> usize {
    match data_type {
        IndoorBikeDataType::Speed => FLAGS_SIZE,
        IndoorBikeDataType::Cadence => {
            let mut index = FLAGS_SIZE;

            if is_speed_present(data) {
                index += INSTANTANEOUS_SPEED_SIZE;
            }

            if is_ave_speed_present(data) {
                index += AVERAGE_SPEED_SIZE;
            }

            index
        }
        IndoorBikeDataType::Distance => {
            let mut index = get_data_index(data, IndoorBikeDataType::Cadence);

            if is_cadence_present(data) {
                index += INSTANTANEOUS_CADENCE_SIZE;
            }

            if is_ave_cadence_present(data) {
                index += AVERAGE_CADENCE_SIZE;
            }

            index
        }
        IndoorBikeDataType::Power => {
            let mut index = get_data_index(data, IndoorBikeDataType::Distance);

            if is_distance_present(data) {
                index += TOTAL_DISTANCE_SIZE;
            }

            if is_resistance_present(data) {
                index += RESISTANCE_LEVEL_SIZE;
            }

            index
        }
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

fn is_ave_cadence_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b1000 == 0b1000
}

fn is_distance_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b10000 == 0b10000
}

fn is_resistance_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b100000 == 0b100000
}

fn is_power_present(data: &Vec<u8>) -> bool {
    let flags = get_flags(data);

    flags & 0b1000000 == 0b1000000
}

// Flags field
// 0 - More data
// 1 - Average speed present
// 2 - Instantaneous cadence present
fn get_flags(data: &Vec<u8>) -> u16 {
    combine_u8_to_u16(data[0], data[1])
}

fn get_time_change() -> Duration {
    static mut LAST_CALL: Option<Instant> = None;

    let now = Instant::now();

    let last_time = unsafe { LAST_CALL.replace(now).unwrap_or(now) };

    now - last_time
}
