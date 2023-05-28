use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, PeripheralId};
use futures::{Stream, StreamExt};
use log::{error, info, warn};
use std::pin::Pin;
use tokio::sync::{Mutex, RwLock};

lazy_static! {
    pub static ref BLUETOOTH: RwLock<Option<Bluetooth>> = Default::default();
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
                let bluetooth_guard = BLUETOOTH.read().await;
                let Some(bt) = bluetooth_guard.as_ref() else {
                    continue;
                };

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

                if let None = properties.local_name.as_ref() {
                    continue;
                }

                let mut devices = bt.devices.write().await;

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

pub struct BTDevice {
    pub id: PeripheralId,
    pub local_name: String,
}

pub struct Bluetooth {
    manager: Mutex<Option<Manager>>,
    status: Mutex<BluetoothStatus>,
    is_scanning: RwLock<bool>,

    pub devices: RwLock<Vec<BTDevice>>,
    pub central: RwLock<Option<Adapter>>,
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
            status: Mutex::new(status),
            is_scanning: RwLock::new(false),
            central: RwLock::new(central),
            devices: RwLock::new(Vec::new()),
        };

        *BLUETOOTH.write().await = Some(bluetooth);

        listen_to_events().await;
    }

    pub async fn start_scan(&self) -> Result<(), String> {
        if *self.is_scanning.read().await {
            info!("Bluetooth is already scanning.");
            return Ok(());
        }

        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            return Err("No Adapter found".into());
        };

        if let Err(e) = central.start_scan(ScanFilter::default()).await {
            error!("Error: {}", e);
            return Err("Bluetooth is unable to scan".into());
        }

        *self.is_scanning.write().await = true;

        info!("Scanning for devices...");

        Ok(())
    }

    pub async fn stop_scan(&self) -> Result<(), String> {
        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            return Err("No Adapter found".into());
        };

        if let Err(e) = central.stop_scan().await {
            error!("Error: {}", e);
            return Err("Bluetooth is unable to stop scan".into());
        }

        *self.is_scanning.write().await = false;

        let mut devices = self.devices.write().await;

        unsafe {
            devices.set_len(0);
        }

        Ok(())
    }

    pub async fn get_scanned_devices(&self) -> Vec<(String, String)> {
        self.devices
            .read()
            .await
            .iter()
            .map(|device| (device.id.clone().to_string(), device.local_name.to_string()))
            .collect()
    }

    pub async fn is_scanning(&self) -> bool {
        *self.is_scanning.read().await
    }
}
