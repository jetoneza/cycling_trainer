use btleplug::api::{Central, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use log::{info, warn};
use std::fmt;
use tokio::sync::{Mutex, RwLock};

use crate::error::error_generic;
use crate::prelude::*;

use super::constants::{
    CYCLING_POWER_SERVICE_UUID, FITNESS_MACHINE_SERVICE_UUID, HEART_RATE_MEASUREMENT_UUID,
    HEART_RATE_SERVICE_UUID, INDOOR_BIKE_DATA_UUID, SPEED_CADENCE_SERVICE_UUID,
};
use super::event_handlers::{
    handle_characteristic_subscription, handle_cycling_device_notifications,
    handle_heart_rate_notifications, listen_to_events, CharacteristicAction,
};
use super::utils::{get_central, get_device_type, get_manager};

lazy_static! {
    pub static ref BLUETOOTH: RwLock<Option<Bluetooth>> = Default::default();
}

const LOGGER_NAME: &str = "ble::bluetooth";

pub enum BluetoothStatus {
    Error,
    Ready,
}

pub enum Connection {
    Connect,
    Disconnect,
}

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

#[derive(Clone, serde::Serialize)]
pub struct BTDevice {
    pub id: String,
    pub local_name: String,
}

pub struct Bluetooth {
    is_scanning: RwLock<bool>,

    pub central: RwLock<Option<Adapter>>,
    pub manager: Mutex<Option<Manager>>,
    pub status: Mutex<BluetoothStatus>,

    pub heart_rate_device: RwLock<Option<Peripheral>>,
    pub cycling_device: RwLock<Option<Peripheral>>,
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
            status: Mutex::new(status),
            heart_rate_device: RwLock::new(None),
            cycling_device: RwLock::new(None),
        };

        *BLUETOOTH.write().await = Some(bluetooth);

        listen_to_events().await;
    }

    pub async fn start_scan(&self, scan_filter: DeviceType) -> Result<()> {
        if *self.is_scanning.read().await {
            info!("{}::start_scan: Bluetooth is already scanning", LOGGER_NAME);
            return Ok(());
        }

        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            return Err(error_generic("No adapater found"));
        };

        let filter = match scan_filter {
            DeviceType::HeartRate => ScanFilter {
                services: vec![HEART_RATE_SERVICE_UUID],
            },
            DeviceType::SmartTrainer => ScanFilter {
                services: vec![
                    FITNESS_MACHINE_SERVICE_UUID,
                    SPEED_CADENCE_SERVICE_UUID,
                    CYCLING_POWER_SERVICE_UUID,
                ],
            },
            DeviceType::Generic => ScanFilter::default(),
        };

        if let Err(e) = central.start_scan(filter).await {
            let message = format!("Bluetooth is unable to start scan: {}", e);
            return Err(error_generic(message.as_str()));
        }

        *self.is_scanning.write().await = true;

        info!("{}::start_scan:: Scanning for devices...", LOGGER_NAME);

        Ok(())
    }

    pub async fn stop_scan(&self) -> Result<()> {
        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            return Err(error_generic("No adapater found"));
        };

        if let Err(e) = central.stop_scan().await {
            let message = format!("Bluetooth is unable to stop scan: {}", e);
            return Err(error_generic(message.as_str()));
        }

        *self.is_scanning.write().await = false;

        Ok(())
    }

    pub async fn handle_connection(&self, id: &str, action: &Connection) -> Result<()> {
        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            return Err(error_generic("No adapater found"));
        };

        let Ok(peripherals) = central.peripherals().await else {
            return Err(error_generic("No peripherals found"));
        };

        let Some(peripheral) = peripherals.iter().find(|p| p.id().to_string() == id) else {
            return Err(error_generic("No device found"));
        };

        let Ok(is_connected) = peripheral.is_connected().await else {
            return Err(error_generic("Unable to determine connection status"));
        };

        let Ok(Some(properties)) = peripheral.properties().await else {
            return Err(error_generic("Unable to get peripheral properties"));
        };

        let device_type = get_device_type(properties.services);

        match (action, is_connected) {
            (Connection::Connect, false) => self.add_device(peripheral.clone(), device_type).await,
            (Connection::Disconnect, true) => {
                self.remove_device(peripheral.clone(), device_type).await
            }
            _ => Ok(()),
        }
    }

    async fn add_device(&self, peripheral: Peripheral, device_type: DeviceType) -> Result<()> {
        if let None = peripheral.connect().await.ok() {
            return Err(error_generic("Unable to connect to device"));
        }

        let Ok(_) = peripheral.discover_services().await else {
            return Err(error_generic("Unable to discover heart rate services"));
        };

        match device_type {
            DeviceType::HeartRate => {
                handle_characteristic_subscription(
                    HEART_RATE_MEASUREMENT_UUID,
                    &peripheral,
                    CharacteristicAction::Subscribe,
                )
                .await?;

                *self.heart_rate_device.write().await = Some(peripheral);

                tokio::spawn(handle_heart_rate_notifications());
            }
            DeviceType::SmartTrainer => {
                handle_characteristic_subscription(
                    INDOOR_BIKE_DATA_UUID,
                    &peripheral,
                    CharacteristicAction::Subscribe,
                )
                .await?;

                // TODO: Add support for separate device. e.g. cycling power + speed and cadence
                // TODO: If indoor bike data is unavailable, use cycling power, speed and cadence

                // TODO: Subcribe to Fitness Machine Control indication?
                // TODO: Request control to Fitness Machine Control

                // TODO: Setup user control for Wahoo (custom characteristic), e.g. setting of weight

                *self.cycling_device.write().await = Some(peripheral);

                tokio::spawn(handle_cycling_device_notifications());
            }
            _ => {}
        };

        Ok(())
    }

    async fn remove_device(&self, peripheral: Peripheral, device_type: DeviceType) -> Result<()> {
        match device_type {
            DeviceType::HeartRate => {
                let hrm_guard = self.heart_rate_device.read().await;
                let Some(hrm) = hrm_guard.as_ref() else {
                    return Err(error_generic("Unable to read heart rate measurement device"))
                };

                handle_characteristic_subscription(
                    HEART_RATE_MEASUREMENT_UUID,
                    &hrm,
                    CharacteristicAction::Unsubscribe,
                )
                .await?;

                drop(hrm_guard);

                *self.heart_rate_device.write().await = None;
            }
            DeviceType::SmartTrainer => {
                let cd_guard = self.heart_rate_device.read().await;
                let Some(cycling_device) = cd_guard.as_ref() else {
                    return Err(error_generic("Unable to read cycling device"))
                };

                handle_characteristic_subscription(
                    INDOOR_BIKE_DATA_UUID,
                    &cycling_device,
                    CharacteristicAction::Unsubscribe,
                )
                .await?;

                drop(cd_guard);

                *self.cycling_device.write().await = None;
            }
            _ => {}
        }

        if let None = peripheral.disconnect().await.ok() {
            return Err(error_generic("Cannot connect device"));
        }

        Ok(())
    }

    pub async fn get_connected_devices(&self) -> Vec<(String, String, String)> {
        let mut devices: Vec<(String, String, String)> = vec![];

        let central_guard = self.central.read().await;
        let Some(central) = central_guard.as_ref() else {
            warn!("{}::get_connected_devices: Adapter not found when getting connected devices", LOGGER_NAME);
            return devices;
        };

        let Ok(peripherals) = central.peripherals().await else {
            warn!("{}::get_connected_devices: Peripherals not found when getting connected devices", LOGGER_NAME);
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
