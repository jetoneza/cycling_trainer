use btleplug::api::{Central, CentralEvent, Peripheral as _, WriteType};
use btleplug::platform::Peripheral;
use futures::{Stream, StreamExt};
use log::{error, info};
use std::fmt;
use std::pin::Pin;
use tauri::{AppHandle, Manager as _};
use uuid::Uuid;

use crate::data::heart_rate_measurement::parse_hrm_data;
use crate::data::indoor_bike_data::parse_indoor_bike_data;
use crate::error::error_generic;
use crate::prelude::*;
use crate::utils::bluetooth_utils::get_uuid_characteristic;
use crate::utils::byte::combine_u8_to_u16;
use crate::utils::code_values::{
    convert_u8_to_ftms_control_op_code_enum, convert_u8_to_ftms_status_code_enum,
    convert_u8_to_spin_down_status_code_enum, convert_u8_to_stop_control_code_enum,
};
use crate::TAURI_APP_HANDLE;

use super::bluetooth::{BTDevice, BluetoothStatus, BLUETOOTH};
use super::constants::{
    FTMSControlOpCode, FTMSControlResultCode, FTMSStatusCode, SpinDownStatus, StopControl,
};

const LOGGER_NAME: &str = "ble::event_handlers";

pub enum CharacteristicAction {
    Subscribe,
    Unsubscribe,
}

impl fmt::Display for CharacteristicAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharacteristicAction::Subscribe => write!(f, "Subscribe"),
            CharacteristicAction::Unsubscribe => write!(f, "Unsubscribe"),
        }
    }
}

pub enum Characteristic {
    CyclingPowerMeasurement,
    HeartRateMeasurement,
    IndoorBikeData,
    FitnessMachineControlPoint,
    FitnessmachineStatus,
    Unknown,
}

pub async fn handle_events(mut events: Pin<Box<dyn Stream<Item = CentralEvent> + Send>>) {
    let bluetooth_guard = BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        return;
    };

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let central_guard = bt.central.read().await;
                let Some(central) = central_guard.as_ref() else {
                    continue;
                };

                let Ok(peripheral) = central.peripheral(&id).await else {
                    continue;
                };

                let Ok(Some(properties)) = peripheral.properties().await else {
                    continue;
                };

                let Some(local_name) = properties.local_name.as_ref() else {
                    continue;
                };

                let Ok(is_connected) = peripheral.is_connected().await else {
                    continue;
                };

                info!(
                    "{}::handle_events: Device discovered: {} - {}",
                    LOGGER_NAME, id, local_name
                );

                if is_connected {
                    continue;
                }

                if let Some(app_handle) = TAURI_APP_HANDLE.lock().await.as_ref() {
                    app_handle
                        .emit_all(
                            "device_discovered",
                            BTDevice {
                                id: id.to_string(),
                                local_name: local_name.to_string(),
                            },
                        )
                        .ok();
                }
            }
            CentralEvent::DeviceConnected(id) => {
                info!("Connected: {}", id);
            }
            CentralEvent::DeviceDisconnected(id) => {
                info!("Disconnected: {}", id);
            }
            _ => {}
        }
    }
}

pub async fn listen_to_events() {
    let bluetooth_guard = BLUETOOTH.read().await;
    let Some(bluetooth) = bluetooth_guard.as_ref() else {
        error!("{}::listen_to_events: Bluetooth not found", LOGGER_NAME);
        return;
    };

    let central_guard = bluetooth.central.read().await;
    let Some(central) = central_guard.as_ref() else {
        error!("{}::listen_to_events: Adapter not found", LOGGER_NAME);
        return;
    };

    let events = match central.events().await {
        Ok(events) => events,
        Err(e) => {
            error!(
                "{}::listen_to_events: Could not detect adapter events: {}",
                LOGGER_NAME, e
            );

            *bluetooth.manager.lock().await = None;
            *bluetooth.central.write().await = None;
            *bluetooth.status.lock().await = BluetoothStatus::Error;

            return;
        }
    };

    drop(central_guard);

    tokio::spawn(handle_events(events));
}

pub async fn handle_heart_rate_notifications() {
    let bluetooth_guard = BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        error!("{}::handle_heart_rate_notifications: Bluetooth not found", LOGGER_NAME);
        return;
    };

    let hrm_guard = bt.heart_rate_device.read().await;
    let Some(hrm) = hrm_guard.as_ref() else {
        error!("{}::handle_heart_rate_notifications: Heart rate measurement device not found", LOGGER_NAME);
        return;
    };

    let Ok(mut notification_stream) = hrm.notifications().await else {
        error!("{}::handle_heart_rate_notifications: Notifications for heart rate measurement not found", LOGGER_NAME);
        return;
    };

    drop(hrm_guard);

    while let Some(data) = notification_stream.next().await {
        let data = parse_hrm_data(&data.value);

        if let Some(app_handle) = TAURI_APP_HANDLE.lock().await.as_ref() {
            app_handle.emit_all("hrm_notification", data).ok();
        }
    }
}

pub async fn handle_cycling_device_notifications() {
    let bluetooth_guard = BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        error!("{}::handle_cycling_device_notifications: Bluetooth not found", LOGGER_NAME);
        return;
    };

    let cd_guard = bt.cycling_device.read().await;
    let Some(cycling_device) = cd_guard.as_ref() else {
        error!("{}::handle_cycling_device_notifications: Cycling device not found", LOGGER_NAME);
        return;
    };

    let Ok(mut notification_stream) = cycling_device.notifications().await else {
        error!("{}::handle_cycling_device_notifications: Notification for cycling device not found", LOGGER_NAME);
        return;
    };

    drop(cd_guard);

    while let Some(data) = notification_stream.next().await {
        let app_handle_guard = TAURI_APP_HANDLE.lock().await;
        let Some(app_handle) = app_handle_guard.as_ref() else {
            error!("{}::handle_cycling_device_notifications: Tauri app handle not found", LOGGER_NAME);
            return;
        };

        match get_uuid_characteristic(data.uuid) {
            Characteristic::IndoorBikeData => {
                let data = parse_indoor_bike_data(&data.value);

                app_handle.emit_all("indoor_bike_notification", data).ok();
            }
            Characteristic::FitnessMachineControlPoint => {
                let status = data.value[0];

                if status != FTMSControlOpCode::Sucess as u8 {
                    return;
                }

                handle_control_point_response(&data.value, &app_handle);
            }
            Characteristic::FitnessmachineStatus => handle_ftms_status(&data.value, &app_handle),
            // TODO: Add support for cycling power
            _ => {}
        };
    }
}

pub async fn handle_characteristic_subscription(
    uuid: Uuid,
    peripheral: &Peripheral,
    action: CharacteristicAction,
) -> Result<()> {
    for characteristic in peripheral.characteristics() {
        if characteristic.uuid != uuid {
            continue;
        }

        let result = match action {
            CharacteristicAction::Subscribe => peripheral.subscribe(&characteristic).await,
            CharacteristicAction::Unsubscribe => peripheral.unsubscribe(&characteristic).await,
        };

        let Ok(_) = result else {
          let message = format!("Unable to {} to characteristic", action);
          return Err(error_generic(message.as_str()));
        };
    }

    Ok(())
}

pub async fn write_to_characteristic(
    uuid: Uuid,
    peripheral: &Peripheral,
    data: &[u8],
    write_type: WriteType,
) -> Result<()> {
    let characteristics = peripheral.characteristics();
    let Some(characteristic) = characteristics.iter().find(|c| c.uuid == uuid) else {
        let message = format!("{}::write_to_characterisic: Unable to find characteristic", LOGGER_NAME);
        return Err(error_generic(message.as_str()))
    };

    let Ok(_) = peripheral
        .write(&characteristic, data, write_type)
        .await else {
            let message = format!("{}::write_to_characterisic: Unable to write to characteristic", LOGGER_NAME);
            return Err(error_generic(message.as_str()));
        };

    Ok(())
}

pub fn handle_control_point_response(data: &Vec<u8>, app_handle: &AppHandle) {
    let request_op_code = data[1];
    let result_code = data[2];

    // TODO: Handle failure

    match convert_u8_to_ftms_control_op_code_enum(request_op_code) {
        FTMSControlOpCode::SpinDownControl => {
            if result_code != FTMSControlResultCode::Success as u8 {
                return;
            }

            let target_speed = combine_u8_to_u16(data[5], data[6]);

            app_handle.emit_all("spin_down_start", target_speed).ok();
        }
        _ => {}
    };
}

pub fn handle_ftms_status(data: &Vec<u8>, app_handle: &AppHandle) {
    let status_code = data[0];

    // TODO: Handle failure

    match convert_u8_to_ftms_status_code_enum(status_code) {
        FTMSStatusCode::SpinDownStatus => {
            let spin_down_status = data[1];

            match convert_u8_to_spin_down_status_code_enum(spin_down_status) {
                SpinDownStatus::StopPedaling => {
                    app_handle.emit_all("spin_down_stop_pedaling", true).ok();
                }
                SpinDownStatus::Success => {
                    let spin_down_time = combine_u8_to_u16(data[2], data[3]);

                    app_handle
                        .emit_all("spin_down_success", spin_down_time)
                        .ok();
                }
                _ => {}
            }
        }
        FTMSStatusCode::StartedOrResumed => {
            app_handle.emit_all("session_started", true).ok();
        }
        FTMSStatusCode::StoppedOrPaused => {
            let stop_control_code = data[1];

            let action = match convert_u8_to_stop_control_code_enum(stop_control_code) {
                StopControl::Stop => "stop",
                StopControl::Pause => "pause",
            };

            app_handle.emit_all("session_stopped", action).ok();
        }
        _ => {}
    }
}
