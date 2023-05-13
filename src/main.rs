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

async fn list_devices() -> Result<(), Box<dyn Error>> {
    let central = get_central().await?;
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

    Ok(())
}

async fn find_trainer(central: &Adapter, trainer_name: &str) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if let Some(local_name) = p.properties().await.unwrap().unwrap().local_name {
            if local_name == trainer_name {
                println!("Trainer {} found!", local_name);
                return Some(p);
            }
        };
    }

    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let central = get_central().await?;
    central.start_scan(ScanFilter::default()).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    let trainer_name = "KICKR CORE AF30";
    let trainer = find_trainer(&central, &trainer_name).await;

    println!("Peripheral for {}: {:?}", trainer_name, trainer);

    list_devices().await?;

    Ok(())
}
