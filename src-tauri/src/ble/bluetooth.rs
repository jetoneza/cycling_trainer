use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use serde::Serialize;
use std::error::Error;
use std::println;
use std::time::Duration;

async fn get_central() -> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await.unwrap();

    if adapters.is_empty() {
        eprintln!("No Bluetooth adapters found!");
    }

    let central = adapters.into_iter().nth(0).unwrap();

    Ok(central)
}

pub struct Bluetooth {
    central: Option<Adapter>,
    // Connected devices
    devices: Vec<Peripheral>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PeripheralData {
    pub name: String,
}

impl Bluetooth {
    pub async fn new() -> Bluetooth {
        let central = get_central().await;

        let central = match central {
            Ok(central) => Some(central),
            Err(_) => {
                println!("No adapter found.");
                None
            }
        };

        Bluetooth {
            central,
            devices: vec![],
        }
    }

    pub async fn list_devices(&self) -> Vec<PeripheralData> {
        let mut data: Vec<PeripheralData> = Vec::new();

        if let Some(central) = self.central.as_ref() {
            central.start_scan(ScanFilter::default()).await.unwrap();

            tokio::time::sleep(Duration::from_secs(2)).await;

            let peripherals = central.peripherals().await.unwrap();

            for p in peripherals {
                if let Some(local_name) = p.properties().await.unwrap().unwrap().local_name {
                    data.push(PeripheralData { name: local_name });
                }
            }
        }

        data
    }
}
