// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ble;

use ble::bluetooth::Bluetooth;

#[tauri::command]
async fn find_device(query: &str) -> Result<String, ()> {
    let mut bluetooth = Bluetooth::new().await;

    bluetooth.start();

    let device = bluetooth.find_device(query).await;

    Ok(format!("Peripheral for {}: {:?}", query, device))
}

// Pseudo code
// const connect_to_device = async (deviceID: string) => {
//   const bluetooth = Bluetooth.getInstance()
//
//   bluetooth.connect(deviceID)
// }

#[tauri::command]
async fn list_devices() -> Result<String, ()> {
    let bluetooth = Bluetooth::new().await;

    // Pseudo code
    // const devices_thread = bluetooth.list_devices()
    //
    // devices_thread.subscribe((data) => {
    //      ui.send(data, 'bluetooth-component')
    // })

    Ok(format!("Devices:"))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_device, list_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
