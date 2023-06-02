use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, PeripheralId};
use futures::{Stream, StreamExt};
use log::{error, info, warn};
use std::pin::Pin;
use tokio::sync::broadcast::{self, Receiver, Sender};
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

                let local_name = match properties.local_name.as_ref() {
                    Some(local_name) => local_name.clone(),
                    None => "".into(),
                };

                info!("Device found: {} {}", id, local_name);

                let Some(sender) = &*bt.scan_broadcast_sender.read().await else {
                    continue;
                };

                if let Err(err) = sender.send(BTDevice {
                    id: id.clone(),
                    local_name,
                }) {
                    warn!("Error broadcasting device: {}", err);
                    continue;
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

#[derive(Clone)]
pub struct BTDevice {
    pub id: PeripheralId,
    pub local_name: String,
}

pub struct Bluetooth {
    manager: Mutex<Option<Manager>>,
    status: Mutex<BluetoothStatus>,
    is_scanning: RwLock<bool>,
    scan_broadcast_sender: RwLock<Option<Sender<BTDevice>>>,

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
            scan_broadcast_sender: RwLock::new(None),
            central: RwLock::new(central),
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

        let (tx, _) = broadcast::channel(1);

        *self.scan_broadcast_sender.write().await = Some(tx);

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

        *self.scan_broadcast_sender.write().await = None;

        Ok(())
    }

    pub async fn get_scan_receiver(&self) -> Option<Receiver<BTDevice>> {
        let Some(tx) = &*self.scan_broadcast_sender.read().await else {
            return None;
        };

        let receiver = tx.subscribe();

        Some(receiver)
    }

    pub async fn connect_to_device(&self, id: String) -> Result<(), String> {
        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            return Err("Adapter not found".into());
        };

        let peripheral_id = PeripheralId(id);

        // TODO: Fix, not working...

        let Ok(peripheral) = central.peripheral(&peripheral_id).await else {
            return Err("Device not found.".into());
        };

        match peripheral.connect().await {
            Err(_) => {
                return Err("Can't connect to device".into());
            }
            _ => Ok(()),
        }
    }
}
