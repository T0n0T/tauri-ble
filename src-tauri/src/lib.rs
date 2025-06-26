// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::Emitter;

mod ota;
mod transfer;
use ota::{Ota, sample::SampleOta};
use transfer::{Transfer, ble::BleTransfer};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValveForm {
  model: String,
  count: u32,
  dir: bool,
}

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Serialize)]
pub struct ValveVal {
  total_ticks: i32,
  position: i32,
}

#[tauri::command]
async fn submit_valve_form(form_data: ValveForm) -> Result<(), String> {
  let payload = serde_json::to_string(&form_data)
    .inspect(|json| println!("Serialized JSON: {}", json))
    .map(|json| format!("config_write {}\r\n", json)) // 拼接前缀
    .map_err(|e| format!("Serialization failed: {}", e))?;

  let ble_transfer = BleTransfer::new();
  ble_transfer.send(payload.as_bytes()).await
}

#[tauri::command]
async fn start_valve_info(app_handle: tauri::AppHandle) -> Result<(), String> {
  let ble_transfer = BleTransfer::new();
  ble_transfer
    .subcribe(Arc::new(move |data: Vec<u8>| {
      Box::pin(async move {
        let app_handle = app_handle.clone(); // 克隆 app_handle
        if data.len() != std::mem::size_of::<ValveVal>() {
          eprintln!(
            "Received data length mismatch. Expected {}, got {}",
            std::mem::size_of::<ValveVal>(),
            data.len()
          );
        }
        let valve_info: ValveVal = *bytemuck::from_bytes::<ValveVal>(&data);
        println!("Valve Info: {:?}", valve_info);
        if let Err(e) = app_handle.emit("valve_info", valve_info) {
          eprintln!("Failed to emit valve info: {}", e);
        }
      })
    }))
    .await
    .map_err(|e| format!("Failed to start valve info: {}", e))?;

  ble_transfer.send("valve_info 1\r\n".as_bytes()).await
}

#[tauri::command]
async fn stop_valve_info() -> Result<(), String> {
  let ble_transfer = BleTransfer::new();
  ble_transfer.unsubscribe().await?;
  ble_transfer.send("valve_info 0\r\n".as_bytes()).await
}

#[tauri::command]
async fn reboot_valve() -> Result<(), String> {
  let ble_transfer = BleTransfer::new();
  ble_transfer.send("reboot\r\n".as_bytes()).await
}

#[tauri::command]
async fn start_valve_ota(app_handle: tauri::AppHandle, file_path: String) -> Result<(), String> {
  let ota_impl = SampleOta::new();
  let handler =
    tauri_plugin_blec::get_handler().map_err(|e| format!("BLE handler unavailable: {:?}", e))?;

  let connected_device = handler
    .connected_device()
    .await
    .map_err(|e| format!("Failed to get connected device: {:?}", e))?;

  let device_address = connected_device.address;

  ota_impl
    .start_ota(app_handle, file_path, device_address)
    .await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_blec::init())
    .invoke_handler(tauri::generate_handler![
      submit_valve_form,
      start_valve_info,
      stop_valve_info,
      reboot_valve,
      start_valve_ota
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
