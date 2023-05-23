// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ble;

use std::time::Duration;

use ble::bluetooth::Bluetooth;
use tauri::Manager;

// Pseudo code
// const connect_to_device = async (deviceID: string) => {
//   const bluetooth = Bluetooth.getInstance()
//
//   bluetooth.connect(deviceID)
// }

#[tauri::command]
async fn list_devices(app_handle: tauri::AppHandle) -> Result<(), ()> {
    let bluetooth = Bluetooth::new().await;

    let mut counter = 0;

    loop {
        if counter > 30 {
            break;
        } 

        let devices = bluetooth.list_devices().await;

        app_handle.emit_all("devices-discovered", devices).ok();

        tokio::time::sleep(Duration::from_secs(5)).await;

        counter += 1;
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
