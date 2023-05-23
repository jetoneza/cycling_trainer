// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

mod ble;

#[tauri::command]
async fn scan_devices() -> Result<(), ()> {
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
