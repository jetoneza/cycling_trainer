use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, PeripheralId};
use futures::{Stream, StreamExt};
use log::{error, info, warn};
use std::pin::Pin;
use tokio::sync::Mutex;

lazy_static! {
    static ref BLUETOOTH: Mutex<Option<Bluetooth>> = Default::default();
}

enum BluetoothStatus {
    Error,
    Ready,
}

async fn get_central(manager: &Option<Manager>) -> Option<Adapter> {
    let adapters = match manager.as_ref().unwrap().adapters().await {
        Ok(adapters) => adapters,
        Err(e) => {
            error!("No bluetooth adapters found: {}", e);
            return None;
        }
    };

    if adapters.is_empty() {
        warn!("No adapters empty.");
    }

    let central = adapters.into_iter().next().unwrap();

    Some(central)
}

async fn get_manager() -> Option<Manager> {
    match Manager::new().await {
        Ok(manager) => Some(manager),
        Err(e) => {
            error!("Could not initialize bluetooth manager: {}", e);
            None
        }
    }
}

async fn handle_events(mut events: Pin<Box<dyn Stream<Item = CentralEvent> + Send>>) {
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let bluetooth_guard = BLUETOOTH.lock().await;
                let bluetooth = match bluetooth_guard.as_ref() {
                    Some(bluetooth) => bluetooth,
                    None => continue,
                };

                let central_guard = bluetooth.central.lock().await;
                let central = match central_guard.as_ref() {
                    Some(central) => central,
                    None => continue,
                };

                let peripheral = match central.peripheral(&id).await {
                    Ok(peripheral) => peripheral,
                    Err(_) => continue,
                };

                let properties = match peripheral.properties().await {
                    Ok(properties) => match properties {
                        Some(properties) => properties,
                        None => continue,
                    },
                    Err(_) => continue,
                };

                if let None = properties.local_name.as_ref() {
                    continue;
                }

                let mut device_ids = bluetooth.device_ids.lock().await;

                if !device_ids.contains(&id) {
                    device_ids.push(id.clone());

                    info!(
                        "Device found: {} {}",
                        properties.local_name.as_ref().unwrap(),
                        &id
                    )
                }
            }
            _ => {}
        }
    }
}

async fn listen_to_events() {
    let bluetooth_guard = BLUETOOTH.lock().await;
    let bluetooth = bluetooth_guard.as_ref().unwrap();

    let central_guard = bluetooth.central.lock().await;
    let central = central_guard.as_ref().unwrap();

    let events = match central.events().await {
        Ok(events) => events,
        Err(e) => {
            error!("Could not detect adapter events: {}", e);

            *bluetooth.manager.lock().await = None;
            *bluetooth.central.lock().await = None;
            *bluetooth.status.lock().await = BluetoothStatus::Error;

            return;
        }
    };

    drop(central_guard);

    tokio::spawn(handle_events(events));
}

pub struct Bluetooth {
    manager: Mutex<Option<Manager>>,
    central: Mutex<Option<Adapter>>,
    status: Mutex<BluetoothStatus>,
    is_scanning: Mutex<bool>,
    device_ids: Mutex<Vec<PeripheralId>>,
}

impl Bluetooth {
    pub async fn init() {
        let manager = get_manager().await;
        let central = get_central(&manager).await;

        let status = match (&manager, &central) {
            (Some(_), Some(_)) => BluetoothStatus::Ready,
            _ => BluetoothStatus::Error,
        };

        let bluetooth = Self {
            manager: Mutex::new(manager),
            central: Mutex::new(central),
            status: Mutex::new(status),
            is_scanning: Mutex::new(false),
            device_ids: Mutex::new(Vec::new()),
        };

        *BLUETOOTH.lock().await = Some(bluetooth);

        listen_to_events().await;
    }
}
