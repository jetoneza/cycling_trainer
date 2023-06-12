use btleplug::api::bleuuid::uuid_from_u32;
use uuid::Uuid;

// Resource:
// https://www.bluetooth.com/specifications/assigned-numbers/

pub const HEART_RATE_SERVICE_UUID: Uuid = uuid_from_u32(0x180D);
pub const HEART_RATE_MEASUREMENT_UUID: Uuid = uuid_from_u32(0x2A37);

pub const FITNESS_MACHINE_SERVICE_UUID: Uuid = uuid_from_u32(0x1826);
pub const CYCLING_POWER_MEASUREMENT_UUID: Uuid = uuid_from_u32(0x2A63);
