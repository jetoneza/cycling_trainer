use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;

    let central = get_central(&manager).await;

    println!("Starting scan on {}...", central.adapter_info().await?);

    central
        .start_scan(ScanFilter::default())
        .await
        .expect("Can't scan BLE adapter for connected devices!");

    // TODO: Use events to detect new device.
    time::sleep(Duration::from_secs(5)).await;

    let trainer = find_trainer(&central, "KICKR CORE AF30").await.expect("No trainer found");

    trainer.connect().await?;

    trainer.discover_services().await?;

    let characteristics = trainer.characteristics();

    for cmd_char in characteristics.iter() {
        println!("{:?}", cmd_char);
    }

    // TODO: Implement main loop.
    // TODO: Remove sleep time.
    time::sleep(Duration::from_secs(30)).await;

    Ok(())
}

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();

    if adapters.is_empty() {
        eprintln!("No Bluetooth adapters found!");
    }

    adapters.into_iter().nth(0).unwrap()
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
