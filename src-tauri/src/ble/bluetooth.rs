use btleplug::api::{Central, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral, PeripheralId};
use log::{error, info, warn};
use std::fmt;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use crate::ble::constants::{FITNESS_MACHINE_SERVICE_UUID, HEART_RATE_SERVICE_UUID};

use super::constants::HEART_RATE_MEASUREMENT_UUID;
use super::utils::{get_central, get_device_type, get_manager, listen_to_events};

lazy_static! {
    pub static ref BLUETOOTH: RwLock<Option<Bluetooth>> = Default::default();
}

pub enum BluetoothStatus {
    Error,
    Ready,
}

#[derive(Debug)]
pub enum Connection {
    Connect,
    Disconnect,
}

#[derive(Debug)]
pub enum DeviceType {
    HeartRate,
    SmartTrainer,
    Generic,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceType::HeartRate => write!(f, "heart_rate"),
            DeviceType::SmartTrainer => write!(f, "smart_trainer"),
            DeviceType::Generic => write!(f, "generic"),
        }
    }
}

#[derive(Clone)]
pub struct BTDevice {
    pub id: PeripheralId,
    pub local_name: String,
}

pub struct Bluetooth {
    is_scanning: RwLock<bool>,

    pub central: RwLock<Option<Adapter>>,
    pub manager: Mutex<Option<Manager>>,
    pub scan_broadcast_sender: RwLock<Option<Sender<BTDevice>>>,
    pub status: Mutex<BluetoothStatus>,

    pub heart_rate_device: RwLock<Option<Peripheral>>,
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
            central: RwLock::new(central),
            manager: Mutex::new(manager),
            is_scanning: RwLock::new(false),
            scan_broadcast_sender: RwLock::new(None),
            status: Mutex::new(status),
            heart_rate_device: RwLock::new(None),
        };

        *BLUETOOTH.write().await = Some(bluetooth);

        listen_to_events().await;
    }

    pub async fn start_scan(&self, scan_filter: DeviceType) -> Result<(), String> {
        if *self.is_scanning.read().await {
            info!("Bluetooth is already scanning.");
            return Ok(());
        }

        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            return Err("No Adapter found".into());
        };

        let filter = match scan_filter {
            DeviceType::HeartRate => ScanFilter {
                services: vec![HEART_RATE_SERVICE_UUID],
            },
            DeviceType::SmartTrainer => ScanFilter {
                // TODO: Add other services related to smart trainers e.g. Cycling Power
                services: vec![FITNESS_MACHINE_SERVICE_UUID],
            },
            DeviceType::Generic => ScanFilter::default(),
        };

        if let Err(e) = central.start_scan(filter).await {
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

    pub async fn handle_connection(&self, id: &str, action: &Connection) -> Result<(), String> {
        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            return Err("Adapter not found".into());
        };

        let Ok(device_id) = Uuid::parse_str(id) else {
            return Err("Unable to parse device id from string to uuid".into());
        };

        let device_id = PeripheralId::from(device_id);

        let Ok(peripheral) = central.peripheral(&device_id).await else {
            return Err("Device not found.".into());
        };

        let Ok(is_connected) = peripheral.is_connected().await else {
            return Err("Can't determine connection status".into());
        };

        let Ok(Some(properties)) = peripheral.properties().await else {
            return Err("Can't get peripheral properties upon connection.".into());
        };

        let device_type = get_device_type(properties.services);

        match (action, is_connected) {
            (Connection::Connect, false) => self.add_device(peripheral, device_type).await,
            (Connection::Disconnect, true) => self.remove_device(peripheral, device_type).await,
            _ => Ok(()),
        }
    }

    async fn add_device(
        &self,
        peripheral: Peripheral,
        device_type: DeviceType,
    ) -> Result<(), String> {
        if let None = peripheral.connect().await.ok() {
            return Err("Cannot connect device.".into());
        }

        match device_type {
            DeviceType::HeartRate => {
                let Ok(_) = peripheral.discover_services().await else {
                    return Err("Unable to discover heart rate services".into());
                };

                for characteristic in peripheral.characteristics() {
                    if characteristic.uuid != HEART_RATE_MEASUREMENT_UUID {
                        continue;
                    }

                    let Ok(_) = peripheral.subscribe(&characteristic).await else {
                        return Err("Unable to subscribe to heart rate measurement characteristics".into());
                    };
                }

                *self.heart_rate_device.write().await = Some(peripheral);
            }
            DeviceType::SmartTrainer => {
                // TODO: Implement smart trainer subscription
            }
            _ => {}
        };

        Ok(())
    }

    async fn remove_device(
        &self,
        peripheral: Peripheral,
        device_type: DeviceType,
    ) -> Result<(), String> {
        if let None = peripheral.disconnect().await.ok() {
            return Err("Cannot connect device.".into());
        }

        match device_type {
            DeviceType::SmartTrainer => {
                *self.heart_rate_device.write().await = None;
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn get_connected_devices(&self) -> Vec<(String, String, String)> {
        let mut devices: Vec<(String, String, String)> = vec![];

        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            warn!("Adapter not found when getting connected devices.");
            return devices;
        };

        let Ok(peripherals) = central.peripherals().await else {
            warn!("Peripherals not found when getting connected devices.");
            return devices;
        };

        for peripheral in peripherals.iter() {
            let Ok(is_connected) = peripheral.is_connected().await else {
                continue;
            };

            if !is_connected {
                continue;
            }

            let Ok(Some(properties)) = peripheral.properties().await else {
                continue;
            };

            let Some(local_name) = properties.local_name.as_ref() else {
                continue;
            };

            let device_type = get_device_type(properties.services);

            devices.push((
                peripheral.id().clone().to_string(),
                local_name.clone(),
                device_type.to_string(),
            ));
        }

        devices
    }
}
