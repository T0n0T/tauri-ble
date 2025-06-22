// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};

mod transfer;
mod ota;
use ota::{Ota, sample::SampleOta};
use transfer::{Transfer, ble::BleTransfer};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValveForm {
  model: String,
  count: u32,
  dir: bool,
}

#[tauri::command]
async fn submit_valve_form(form_data: ValveForm) -> Result<(), String> {
  let payload = serde_json::to_string(&form_data)
    .inspect(|json| println!("Serialized JSON: {}", json))
    .map(|json| format!("config_write {}\r\n", json)) // 拼接前缀
    .map_err(|e| format!("Serialization failed: {}", e))?;

  let ble_transfer = BleTransfer::new();
  ble_transfer.send_data(payload.as_bytes()).await
}

#[tauri::command]
async fn start_valve_ota(app_handle: tauri::AppHandle, file_path: String) -> Result<(), String> {
    let ota_impl = SampleOta::new();
    let ble_transfer = BleTransfer::new();
    ota_impl.start_ota(app_handle, file_path, ble_transfer).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_blec::init())
    .invoke_handler(tauri::generate_handler![submit_valve_form, start_valve_ota])
    .run(tauri::generate_context!())
    .expect("error while running tauri application"); 
}
