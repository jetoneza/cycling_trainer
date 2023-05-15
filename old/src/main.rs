use std::error::Error;

mod ble;

use ble::bluetooth::Bluetooth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bluetooth = Bluetooth::new().await;

    find_device(&bluetooth, "KICKR CORE AF30").await;
    find_device(&bluetooth, "MX Master 3").await;

    bluetooth.list_devices().await?;

    Ok(())
}

async fn find_device(bluetooth: &Bluetooth, name: &str) {
    let device = bluetooth.find_device(name).await;

    println!("Peripheral for {}: {:?}", name, device);
}
