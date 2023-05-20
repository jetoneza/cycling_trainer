// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ble;

use ble::bluetooth::Bluetooth;

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
        .invoke_handler(tauri::generate_handler![list_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
