use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::error::Error;
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

        Bluetooth { central }
    }

    pub async fn get_central(&self) -> Option<&Adapter> {
        self.central.as_ref()
    }

    pub async fn list_devices(&self) -> Result<(), Box<dyn Error>> {
        if let Some(central) = self.central.as_ref() {
            let mut events = central.events().await?;

            // TODO: Filter out unnecessary devices
            central.start_scan(ScanFilter::default()).await?;

            println!("BLE Devices:");

            while let Some(event) = events.next().await {
                match event {
                    CentralEvent::DeviceDiscovered(id) => {
                        let peripheral = central.peripheral(&id).await?;

                        let properties = peripheral.properties().await?.unwrap();
                        let is_connected = peripheral.is_connected().await?;
                        let address = properties.address;
                        let local_name = properties
                            .local_name
                            .unwrap_or(String::from("Unknown BLE Device"));

                        if is_connected {
                            print!("* ");
                        } else {
                            print!("  ");
                        }

                        print!("{:?} : {:?}", address, local_name);
                        println!("");
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub async fn find_device(&self, device_name: &str) -> Option<Peripheral> {
        println!("Searching for {}...", device_name);

        if let Some(central) = self.central.as_ref() {
            central.start_scan(ScanFilter::default()).await.unwrap();

            tokio::time::sleep(Duration::from_secs(5)).await;

            for p in central.peripherals().await.unwrap() {
                if let Some(local_name) = p.properties().await.unwrap().unwrap().local_name {
                    if local_name == device_name {
                        println!("Device {} found!", local_name);
                        return Some(p);
                    }
                };
            }
        }

        None
    }
}
