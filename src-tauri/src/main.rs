// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ble;

use std::{println, sync::Mutex, time::Duration};

use ble::bluetooth::Bluetooth;
use tauri::{Manager, State};

// Pseudo code
// const connect_to_device = async (deviceID: string) => {
//   const bluetooth = Bluetooth.getInstance()
//
//   bluetooth.connect(deviceID)
// }

struct AppState(Mutex<App>);

struct App {
    is_scanning: bool,
}

impl App {
    fn new() -> Self {
        Self { is_scanning: false }
    }

    fn set_scanning(&mut self, is_scanning: bool) {
        self.is_scanning = is_scanning;
    }

    fn is_scanning(&self) -> bool {
        self.is_scanning
    }
}

#[tauri::command]
async fn stop_scanning(state: State<'_, AppState>) -> Result<(), ()> {
    if let Ok(mut app) = state.0.lock() {
        app.set_scanning(false);
    }

    Ok(())
}

#[tauri::command]
async fn scan_devices(app_handle: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), ()> {
    let bluetooth = Bluetooth::new().await;

    if let Ok(mut app) = state.0.lock() {
        app.set_scanning(true);
    }

    loop {
        if let Ok(app) = state.0.lock() {
            if !app.is_scanning() {
                println!("Scanning stopped!");
                break;
            }
        }

        println!("Scanning...");

        let devices = bluetooth.list_devices().await;

        app_handle.emit_all("devices-discovered", devices).ok();
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(App::new())))
        .invoke_handler(tauri::generate_handler![scan_devices, stop_scanning])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
