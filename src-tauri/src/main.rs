// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod ble;

use ble::bluetooth::Bluetooth;
use log::{error, warn};
use tauri::Manager;
use tokio::sync::Mutex;

use ble::bluetooth::BLUETOOTH;

lazy_static! {
    pub static ref TAURI_APP_HANDLE: Mutex<Option<tauri::AppHandle>> = Default::default();
}

async fn receive_scanned_devices() {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("Bluetooth not found.");
        return;
    };

    let Some(device) = bt.get_scan_recv().await else {
        return;
    };

    if let Some(app_handle) = TAURI_APP_HANDLE.lock().await.as_ref() {
        app_handle
            .emit_all(
                "device-discovered",
                (device.id.to_string(), device.local_name.to_string()),
            )
            .ok();
    }
}

#[tauri::command]
async fn stop_scan() -> Result<(), String> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("Bluetooth not found.");
        return Ok(());
    };

    bt.stop_scan().await?;

    Ok(())
}

#[tauri::command]
async fn start_scan() -> Result<(), String> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("Bluetooth not found.");
        return Ok(());
    };

    bt.start_scan().await?;

    tokio::spawn(receive_scanned_devices());

    Ok(())
}

#[tauri::command]
async fn connect_to_device(device_id: String) -> Result<(), String> {
    todo!();
}

async fn initialize_app(app_handle: tauri::AppHandle) {
    *TAURI_APP_HANDLE.lock().await = Some(app_handle.clone());

    // TODO: Pass instance to tauri
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
