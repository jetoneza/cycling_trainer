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
pub const FITNESS_MACHINE_STATUS_UUID: Uuid = uuid_from_u32(0x2ADA);

// Cycling Power
pub const CYCLING_POWER_SERVICE_UUID: Uuid = uuid_from_u32(0x1818);
pub const CYCLING_POWER_MEASUREMENT_UUID: Uuid = uuid_from_u32(0x2A63);

pub enum FTMSControlOpCode {
    RequestControl = 0x00,
    TargetPower = 0x05,
    Start = 0x07, // Start/Resume
    Stop = 0x08,  // Stop/Pause
    SpinDownControl = 0x13,
    TargetCadence = 0x14,
    Sucess = 0x80,
}

pub enum FTMSControlResultCode {
    Success = 0x01,
    OpCodeNotSupported = 0x02,
    InvalidParameter = 0x03,
    OperationFailed = 0x04,
    ControlNotPermitted = 0x05,
}

pub enum SpinDownControl {
    Start = 0x01,
    Ignore = 0x02,
}

pub enum StopControl {
    Stop = 0x01,
    Pause = 0x02,
}

pub enum SpinDownStatus {
    SpinDownRequested = 0x01,
    Success = 0x02,
    Error = 0x03,
    StopPedaling = 0x04,
}

pub enum FTMSStatusCode {
    StoppedOrPaused = 0x02,
    StoppedBySafetyKey = 0x03,
    StartedOrResumed = 0x04,
    TargetPowerChanged = 0x08,
    SpinDownStatus = 0x14,
    TargetCadenceChanged = 0x15,
    ControlPermissionLost = 0xFF,
}
