use btleplug::api::bleuuid::uuid_from_u32;
use uuid::Uuid;

// Resource:
// https://www.bluetooth.com/specifications/assigned-numbers/

// Heart Rate
pub const HEART_RATE_SERVICE_UUID: Uuid = uuid_from_u32(0x180D);
pub const HEART_RATE_MEASUREMENT_UUID: Uuid = uuid_from_u32(0x2A37);

// Speed and Cadence
pub const SPEED_CADENCE_SERVICE_UUID: Uuid = uuid_from_u32(0x1816);

// Fitness Machine
pub const FITNESS_MACHINE_SERVICE_UUID: Uuid = uuid_from_u32(0x1826);
pub const INDOOR_BIKE_DATA_UUID: Uuid = uuid_from_u32(0x2AD2);
pub const FITNESS_MACHINE_CONTROL_POINT_UUID: Uuid = uuid_from_u32(0x2AD9);
pub const FTMS_CONTROL_REQUEST_CONTROL_OP_CODE: u8 = 0x00;

// Cycling Power
pub const CYCLING_POWER_SERVICE_UUID: Uuid = uuid_from_u32(0x1818);
pub const CYCLING_POWER_MEASUREMENT_UUID: Uuid = uuid_from_u32(0x2A63);
