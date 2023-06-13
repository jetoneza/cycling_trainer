#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct HeartRateMeasurement {
    pub bpm: u16,
    pub is_sensor_contact_supported: bool,
    pub is_sensor_in_contact: bool,
}

// Extracting Heart Rate Measurements from Bluetooth LE Packets
// https://mariam.qa/post/hr-ble/
pub fn parse_hrm_data(data: &Vec<u8>) -> HeartRateMeasurement {
    let flags = data[0];

    // HRM Characteristic
    // | 0  | 1 | 2 | 3  | 4  | 5 | 6 | 7 |
    // | HR |  SC   | EE | RR |   RFFU    |

    // HR Data format: 1-bit that indicates if HR values in the format u8 or u16
    // 0 - u8, 1 - u16
    let is_16 = flags & 1 == 1;

    // Sensor Contact (SC): 2-bits indicating whether SC feature is supported or not, and
    // if supported whether the device in good or poor contact with the skin.
    // 00, 01: SC is not supported
    // 10: SC is supported, but contact is not detected
    // 11: SC is supported and contact is detected
    let is_sensor_contact_supported = flags & 0b100 == 0b100;
    let is_sensor_in_contact = flags & 0b110 == 0b110;

    let bpm = if is_16 {
        u16::from_be_bytes([data[1], data[2]])
    } else {
        data[1] as u16
    };

    // TODO: Complete parsing of data
    // e.g. detection of EE and RR
    HeartRateMeasurement {
        bpm,
        is_sensor_in_contact,
        is_sensor_contact_supported,
    }
}
