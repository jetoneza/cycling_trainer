// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod ble;

use std::{println, time::Duration};

use ble::bluetooth::Bluetooth;
use btleplug::api::{Central, ScanFilter};
use log::{error, info, warn};
use tauri::Manager;
use tokio::sync::Mutex;

use ble::bluetooth::BLUETOOTH;

lazy_static! {
    pub static ref TAURI_APP_HANDLE: Mutex<Option<tauri::AppHandle>> = Default::default();
}

async fn scan_devices(bluetooth: &Bluetooth) -> Result<(), String> {
    loop {
        if !*bluetooth.is_scanning.lock().await {
            info!("Scan finished");
            break;
        }

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

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

#[tauri::command]
async fn stop_scan() -> Result<(), String> {
    let bluetooth_guard = BLUETOOTH.lock().await;
    let Some(bluetooth) = bluetooth_guard.as_ref() else {
        warn!("Bluetooth not found.");
        return Ok(());
    };

    let central_guard = bluetooth.central.lock().await;
    let Some(central) = central_guard.as_ref() else {
        return Err("No Adapter found".into());
    };

    if let Err(e) = central.stop_scan().await {
        error!("Error: {}", e);
        return Err("Unable to stop scanning.".into());
    }

    *bluetooth.is_scanning.lock().await = false;

    Ok(())
}

#[tauri::command]
async fn start_scan() -> Result<(), String> {
    let bluetooth_guard = BLUETOOTH.lock().await;
    let Some(bluetooth) = bluetooth_guard.as_ref() else {
        warn!("Bluetooth not found.");
        return Ok(());
    };

    if *bluetooth.is_scanning.lock().await {
        info!("Blue tooth is already scanning.");
        return Ok(());
    }

    let central_guard = bluetooth.central.lock().await;
    let Some(central) = central_guard.as_ref() else {
        return Err("No Adapter found".into());
    };

    if let Err(e) = central.start_scan(ScanFilter::default()).await {
        error!("Error: {}", e);
        return Err("Bluetooth is unable to scan".into());
    }

    *bluetooth.is_scanning.lock().await = true;

    info!("Scanning for devices...");

    scan_devices(&bluetooth).await?;

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
        .invoke_handler(tauri::generate_handler![
            start_scan,
            stop_scan,
            connect_to_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
