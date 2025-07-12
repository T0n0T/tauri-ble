use super::{ValveConfig, ValveVal, do_request_response};
use crate::transfer::ble::BleTransfer;
use std::sync::Arc;
use tauri::Emitter;

#[tauri::command]
pub async fn valve_configure(config: ValveConfig) -> Result<(), String> {
  let payload = serde_json::to_string(&config)
    .inspect(|json| log::info!("Serialized JSON: {}", json))
    .map(|json| format!("config_write {}\r\n", json)) // 拼接前缀
    .map_err(|e| format!("Serialization failed: {}", e))?;

  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(Arc::new(ble_transfer), &payload, 3, false, None).await
}

#[tauri::command]
pub async fn valve_readconfig(app_handle: tauri::AppHandle) -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(
    Arc::new(ble_transfer),
    "config_read\r\n",
    3,
    false,
    Some(Arc::new(move |data: Vec<u8>| {
      match serde_json::from_slice::<ValveConfig>(&data) {
        Ok(valve_config) => {
          log::debug!("Valve config: {:?}", valve_config);
          if let Err(e) = app_handle.emit("valve_config", valve_config) {
            log::error!("Failed to emit valve config: {}", e);
          }
        }
        Err(e) => {
          log::error!("Failed to parse valve config: {}", e);
        }
      }
    })),
  )
  .await
}

#[tauri::command]
pub async fn valve_refactory() -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(
    Arc::new(ble_transfer),
    "config_refactory\r\n",
    3,
    false,
    None,
  )
  .await
}

#[tauri::command]
pub async fn valve_tuning_start(app_handle: tauri::AppHandle) -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(
    Arc::new(ble_transfer),
    "valve_tuning 1\r\n",
    3,
    true,
    Some(Arc::new(move |data: Vec<u8>| {
      if data.len() != std::mem::size_of::<ValveVal>() {
        log::error!(
          "Received data length mismatch. Expected {}, got {}",
          std::mem::size_of::<ValveVal>(),
          data.len()
        );
        return;
      }
      let valve_info: ValveVal = *bytemuck::from_bytes::<ValveVal>(&data);
      log::debug!("Valve Info: {:?}", valve_info);
      if let Err(e) = app_handle.emit("valve_tuning", valve_info) {
        log::error!("Failed to emit valve info: {}", e);
      }
    })),
  )
  .await
}

#[tauri::command]
pub async fn valve_tuning_stop() -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(
    Arc::new(ble_transfer),
    "valve_tuning 0\r\n",
    3,
    false,
    None,
  )
  .await
}
