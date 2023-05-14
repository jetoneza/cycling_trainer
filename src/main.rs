use btleplug::api::{Central, ScanFilter};
use std::error::Error;
use std::time::Duration;

mod  ble;

use ble::bluetooth::Bluetooth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bluetooth = Bluetooth::new().await;
    let central = bluetooth.get_central().await.unwrap();

    central.start_scan(ScanFilter::default()).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    let trainer_name = "KICKR CORE AF30";
    let trainer = bluetooth.find_trainer("KICKR CORE AF30").await;

    println!("Peripheral for {}: {:?}", trainer_name, trainer);

    bluetooth.list_devices().await?;

    Ok(())
}
