// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ble;

use ble::bluetooth::Bluetooth;

#[tauri::command]
async fn find_device(query: &str) -> Result<String, ()> {
    let bluetooth = Bluetooth::new().await;

    let device = bluetooth.find_device(query).await;

    Ok(format!("Peripheral for {}: {:?}", query, device))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

