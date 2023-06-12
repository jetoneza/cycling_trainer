use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::{Stream, StreamExt};
use log::{error, info, warn};
use std::pin::Pin;
use tauri::Manager as _;
use uuid::Uuid;

use crate::TAURI_APP_HANDLE;

use super::bluetooth::{BTDevice, BluetoothStatus, DeviceType, BLUETOOTH};
use super::constants::{FITNESS_MACHINE_SERVICE_UUID, HEART_RATE_SERVICE_UUID};
use super::heart_rate_measurement::parse_hrm_data;

#[derive(Debug)]
pub enum CharacteristicAction {
    Subscribe,
    Unsubscribe,
}

pub async fn get_central(manager: &Option<Manager>) -> Option<Adapter> {
    let Some(manager) = manager.as_ref() else {
        warn!("No manager found.");
        return None;
    };

    let Ok(adapters) = manager.adapters().await else {
        warn!("No bluetooth adapters found");
        return None;
    };

    adapters.into_iter().next()
}

pub async fn get_manager() -> Option<Manager> {
    match Manager::new().await {
        Ok(manager) => Some(manager),
        Err(e) => {
            error!("Could not initialize bluetooth manager: {}", e);
            None
        }
    }
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

                info!("Device found: {} {} {}", id, local_name, is_connected);

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
                println!("Connected: {}", id);
            }
            CentralEvent::DeviceDisconnected(id) => {
                println!("Disconnected: {}", id);
            }
            _ => {}
        }
    }
}

pub async fn listen_to_events() {
    let bluetooth_guard = BLUETOOTH.read().await;
    let Some(bluetooth) = bluetooth_guard.as_ref() else {
        error!("Can't find bluetooth.");
        return;
    };

    let central_guard = bluetooth.central.read().await;
    let Some(central) = central_guard.as_ref() else {
        error!("Can't find adapter.");
        return;
    };

    let events = match central.events().await {
        Ok(events) => events,
        Err(e) => {
            error!("Could not detect adapter events: {}", e);

            *bluetooth.manager.lock().await = None;
            *bluetooth.central.write().await = None;
            *bluetooth.status.lock().await = BluetoothStatus::Error;

            return;
        }
    };

    drop(central_guard);

    tokio::spawn(handle_events(events));
}

pub fn get_device_type(services: Vec<Uuid>) -> DeviceType {
    let is_heart_rate = services.contains(&HEART_RATE_SERVICE_UUID);
    let is_smart_trainer = services.contains(&FITNESS_MACHINE_SERVICE_UUID);

    match (is_heart_rate, is_smart_trainer) {
        (true, false) => DeviceType::HeartRate,
        (false, true) => DeviceType::SmartTrainer,
        _ => DeviceType::Generic,
    }
}

pub async fn handle_heart_rate_notifications() {
    let bluetooth_guard = BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        error!("Can't find bluetooth.");
        return;
    };

    let hrm_guard = bt.heart_rate_device.read().await;
    let Some(hrm) = hrm_guard.as_ref() else {
        error!("Can't find heart rate measurment device.");
        return;
    };

    let Ok(mut notification_stream) = hrm.notifications().await else {
        error!("No notifications for heart rate measurement.");
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

pub async fn handle_cycling_power_notifications() {}

pub async fn on_characteristic_subscription(
    uuid: Uuid,
    peripheral: &Peripheral,
    action: CharacteristicAction,
) -> Result<(), String> {
    for characteristic in peripheral.characteristics() {
        if characteristic.uuid != uuid {
            continue;
        }

        let result = match action {
            CharacteristicAction::Subscribe => peripheral.subscribe(&characteristic).await,
            CharacteristicAction::Unsubscribe => peripheral.unsubscribe(&characteristic).await,
        };

        let Ok(_) = result else {
          return Err(format!("Unable to {:?} to characteristic", action));
        };
    }

    Ok(())
}
