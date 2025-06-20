// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use uuid::{Uuid, uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValveForm {
  model: String,
  count: u32,
  dir: bool,
}

const READ_CHARACTERISTIC_UUID: Uuid = uuid!("0000fff1-0000-1000-8000-00805f9b34fb");
const WRITE_CHARACTERISTIC_UUID: Uuid = uuid!("0000fff2-0000-1000-8000-00805f9b34fb");

#[tauri::command]
async fn submit_valve_form(form_data: ValveForm) -> Result<(), String> {
  let payload = serde_json::to_string(&form_data)
      .inspect(|json| println!("Serialized JSON: {}", json))
      .map(|json| format!("config_write {}\r\n", json))  // 拼接前缀
      .map_err(|e| format!("Serialization failed: {}", e))?;

  tauri_plugin_blec::get_handler()
      .map_err(|e| format!("BLE handler unavailable: {:?}", e))?
      .send_data(
          WRITE_CHARACTERISTIC_UUID,
          payload.as_bytes(),  
          tauri_plugin_blec::models::WriteType::WithResponse,
      )
      .await
      .map_err(|e| format!("BLE send failed: {:?}", e))?;

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_blec::init())
    .invoke_handler(tauri::generate_handler![submit_valve_form])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
