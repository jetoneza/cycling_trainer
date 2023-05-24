// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod ble;

use std::{time::Duration, println};

use ble::bluetooth::Bluetooth;
use btleplug::api::{Central, ScanFilter};
use log::{error, info};
use tauri::Manager;
use tokio::sync::Mutex;

use ble::bluetooth::BLUETOOTH;

lazy_static! {
    pub static ref TAURI_APP_HANDLE: Mutex<Option<tauri::AppHandle>> = Default::default();
}

#[tauri::command]
async fn scan_devices() -> Result<(), String> {
    let bluetooth_guard = BLUETOOTH.lock().await;
    let bluetooth = match bluetooth_guard.as_ref() {
        Some(bluetooth) => bluetooth,
        None => {
            return Ok(());
        }
    };

    if *bluetooth.is_scanning.lock().await {
        return Ok(());
    }

    if let Some(central) = bluetooth.central.lock().await.as_ref() {
        if let Err(e) = central.start_scan(ScanFilter::default()).await {
            error!("Bluetooth is unable to scan: {}", e);

            return Err("Bluetooth is unable to scan".into());
        }

        *bluetooth.is_scanning.lock().await = true;

        info!("Scanning for devices...");
    } else {
        error!("No adapter found.");
        return Err("No Adapter found".into());
    }

    // Scan for 10 seconds
    tokio::time::sleep(Duration::from_secs(10)).await;

    if let Some(central) = bluetooth.central.lock().await.as_ref() {
        central.stop_scan().await.unwrap();
        info!("Scan finished");
    }

    *bluetooth.is_scanning.lock().await = false;

    if let Some(app_handle) = TAURI_APP_HANDLE.lock().await.as_ref() {
        let devices: Vec<(String, String)> = bluetooth
            .devices
            .lock()
            .await
            .iter()
            .map(|device| (device.id.clone().to_string(), device.local_name.to_string()))
            .collect();

        app_handle.emit_all("devices-discovered", devices).ok();
    }

    Ok(())
}

#[tauri::command]
async fn connect_to_device(device_id: String) -> Result<(), String> {
    println!("{}", device_id);

    Ok(())
}

async fn initialize_app(app_handle: tauri::AppHandle) {
    *TAURI_APP_HANDLE.lock().await = Some(app_handle.clone());

    Bluetooth::init().await;
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            match tauri::async_runtime::block_on(tauri::async_runtime::spawn(initialize_app(
                app.app_handle(),
            ))) {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("Error on initialization!: {}", e);
                    std::process::exit(1);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![scan_devices, connect_to_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
