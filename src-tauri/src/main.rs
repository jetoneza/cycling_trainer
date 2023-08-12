// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod ble;
mod data;
mod error;
mod prelude;
mod utils;
mod workouts;

use crate::prelude::*;

use ble::bluetooth::{Bluetooth, Connection, DeviceType, BLUETOOTH};
use data::session::Session;
use error::error_generic;
use log::{error, warn};
use tauri::Manager;
use tauri_plugin_log::{self, LogTarget};
use tokio::sync::Mutex;
use workouts::activities::{self, Activity, ACTIVITIES};

lazy_static! {
    pub static ref TAURI_APP_HANDLE: Mutex<Option<tauri::AppHandle>> = Default::default();
}

#[tauri::command(async)]
async fn get_connected_devices() -> Result<Vec<(String, String, String)>> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        return Err(error_generic("Bluetooth not found when getting connected devices"));
    };

    let devices = bt.get_connected_devices().await;

    Ok(devices)
}

#[tauri::command(async)]
async fn stop_scan() -> Result<()> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::stop_scan: Bluetooth not found.");
        return Ok(());
    };

    bt.stop_scan().await?;

    Ok(())
}

#[tauri::command(async)]
async fn start_scan(scan_filter: &str) -> Result<()> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::stop_scan: Bluetooth not found.");
        return Ok(());
    };

    let filter = match scan_filter {
        "heart_rate" => DeviceType::HeartRate,
        "smart_trainer" => DeviceType::SmartTrainer,
        _ => DeviceType::Generic,
    };

    bt.start_scan(filter).await?;

    Ok(())
}

#[tauri::command(async)]
async fn connect_device(device_id: &str) -> Result<()> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::stop_scan: Bluetooth not found.");
        return Ok(());
    };

    bt.handle_connection(device_id, &Connection::Connect)
        .await?;

    Ok(())
}

#[tauri::command(async)]
async fn disconnect_device(device_id: &str) -> Result<()> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::stop_scan: Bluetooth not found.");
        return Ok(());
    };

    bt.handle_connection(device_id, &Connection::Disconnect)
        .await?;

    Ok(())
}

#[tauri::command(async)]
async fn get_activities() -> Result<Vec<Activity>> {
    let Some(lock) = ACTIVITIES.get() else {
        return Err(error_generic("main::get_activities: Unable to get ACTIVITIES."));
    };

    let guard = lock.read().await;
    let activities: &Vec<Activity> = guard.as_ref();

    Ok(activities.clone())
}

#[tauri::command(async)]
async fn execute_workout(power: usize, cadence: usize) -> Result<()> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::execute_workout: Bluetooth not found.");
        return Ok(());
    };

    bt.set_target_power(power).await?;
    bt.set_target_cadence(cadence).await?;

    Ok(())
}

#[tauri::command(async)]
async fn request_spin_down() -> Result<()> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::request_spin_down: Bluetooth not found.");
        return Ok(());
    };

    bt.request_spin_down().await?;

    Ok(())
}

#[tauri::command(async)]
async fn start_session() -> Result<()> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::start_session: Bluetooth not found.");
        return Ok(());
    };

    bt.start_session().await?;

    Ok(())
}

#[tauri::command(async)]
async fn stop_session(action: &str) -> Result<()> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::stop_session: Bluetooth not found.");
        return Ok(());
    };

    bt.stop_session(action).await?;

    Ok(())
}

#[tauri::command(async)]
async fn get_session_data() -> Result<Option<Session>> {
    let bluetooth_guard = &BLUETOOTH.read().await;
    let Some(bt) = bluetooth_guard.as_ref() else {
        warn!("main::get_session_data: Bluetooth not found.");
        return Ok(None);
    };

    let session_data = bt.get_session_data().await?;

    Ok(Some(session_data))
}

async fn initialize_app(app_handle: tauri::AppHandle) {
    *TAURI_APP_HANDLE.lock().await = Some(app_handle.clone());

    activities::load_activities();

    Bluetooth::init().await;
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .setup(|app| {
            match tauri::async_runtime::block_on(tauri::async_runtime::spawn(initialize_app(
                app.app_handle(),
            ))) {
                Ok(_) => Ok(()),
                Err(e) => {
                    error!("main: Error on initialization!: {}", e);
                    std::process::exit(1);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            start_scan,
            stop_scan,
            connect_device,
            disconnect_device,
            get_connected_devices,
            get_activities,
            execute_workout,
            request_spin_down,
            start_session,
            stop_session,
            get_session_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
