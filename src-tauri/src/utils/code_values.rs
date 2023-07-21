use crate::ble::constants::{FTMSControlOpCode, FTMSStatusCode, SpinDownStatus, StopControl};

// TODO: improve the following functions

pub fn convert_u8_to_ftms_control_op_code_enum(number: u8) -> FTMSControlOpCode {
    match number {
        0x00..=0x80 => unsafe { std::mem::transmute::<u8, FTMSControlOpCode>(number) },
        _ => panic!("Invalid value"),
    }
}

pub fn convert_u8_to_ftms_status_code_enum(number: u8) -> FTMSStatusCode {
    unsafe { std::mem::transmute::<u8, FTMSStatusCode>(number) }
}

pub fn convert_u8_to_spin_down_status_code_enum(number: u8) -> SpinDownStatus {
    match number {
        0x00..=0x04 => unsafe { std::mem::transmute::<u8, SpinDownStatus>(number) },
        _ => panic!("Invalid value"),
    }
}

pub fn convert_u8_to_stop_control_code_enum(number: u8) -> StopControl {
    match number {
        0x01..=0x02 => unsafe { std::mem::transmute::<u8, StopControl>(number) },
        _ => panic!("Invalid value"),
    }
}
