use crate::error::error_generic;
use crate::prelude::*;

use btleplug::api::{Central, CentralEvent, Peripheral as _};
use btleplug::platform::Peripheral;
use futures::{Stream, StreamExt};
use log::{error, info};
use std::fmt;
use std::pin::Pin;
use tauri::Manager as _;
use uuid::Uuid;

use crate::data::heart_rate_measurement::parse_hrm_data;
use crate::data::indoor_bike_data::parse_indoor_bike_data;
use crate::TAURI_APP_HANDLE;

use super::bluetooth::{BTDevice, BluetoothStatus, BLUETOOTH};
use super::utils::get_uuid_characteristic;

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
                            "device-discovered",
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
            app_handle.emit_all("hrm-notification", data).ok();
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

                app_handle.emit_all("indoor-bike-notification", data).ok();
            }
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
