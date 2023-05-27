use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, PeripheralId};
use futures::{Stream, StreamExt};
use log::{error, info, warn};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref BLUETOOTH: Arc<Mutex<Option<Bluetooth>>> = Default::default();
}

pub enum BluetoothStatus {
    Error,
    Ready,
}

async fn get_central(manager: &Option<Manager>) -> Option<Adapter> {
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
                let Some(bluetooth) = bluetooth_guard.as_ref() else {
                    continue;
                };

                let central_guard = bluetooth.central.lock().await;
                let Some(central) = central_guard.as_ref() else {
                    continue;
                };

                let Ok(peripheral) = central.peripheral(&id).await else {
                    continue;
                };

                let Ok(Some(properties)) = peripheral.properties().await else {
                    continue;
                };

                if let None = properties.local_name.as_ref() {
                    continue;
                }

                let mut devices = bluetooth.devices.lock().await;

                if !devices.iter().any(|device| device.id == id) {
                    let local_name = match properties.local_name.as_ref() {
                        Some(local_name) => local_name.clone(),
                        None => "".into(),
                    };

                    info!("Device found: {} {}", id, local_name);

                    devices.push(BTDevice {
                        id: id.clone(),
                        local_name,
                    });
                }
            }
            _ => {}
        }
    }
}

async fn listen_to_events() {
    let bluetooth_guard = BLUETOOTH.lock().await;
    let Some(bluetooth) = bluetooth_guard.as_ref() else {
        error!("Can't find bluetooth.");
        return;
    };

    let central_guard = bluetooth.central.lock().await;
    let Some(central) = central_guard.as_ref() else {
        error!("Can't find adapter.");
        return;
    };

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

pub struct BTDevice {
    pub id: PeripheralId,
    pub local_name: String,
}

pub struct Bluetooth {
    pub manager: Mutex<Option<Manager>>,
    pub status: Mutex<BluetoothStatus>,
    pub devices: Mutex<Vec<BTDevice>>,
    pub central: Mutex<Option<Adapter>>,
    pub is_scanning: Mutex<bool>,
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
            devices: Mutex::new(Vec::new()),
        };

        *BLUETOOTH.lock().await = Some(bluetooth);

        listen_to_events().await;
    }
}
