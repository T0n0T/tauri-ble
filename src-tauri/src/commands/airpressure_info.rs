use super::{AirPressureVal, do_request_response};
use crate::transfer::ble::BleTransfer;
use std::sync::Arc;
use tauri::Emitter;

#[tauri::command]
pub async fn start_airpressure_info(app_handle: tauri::AppHandle) -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(
    Arc::new(ble_transfer),
    "valve_info 1\r\n",
    3,
    true,
    Some(Arc::new(move |data: Vec<u8>| {
      if data.len() != std::mem::size_of::<AirPressureVal>() {
        log::error!(
          "Received data length mismatch. Expected {}, got {}",
          std::mem::size_of::<AirPressureVal>(),
          data.len()
        );
        return;
      }
      let airpressure_info: AirPressureVal = *bytemuck::from_bytes::<AirPressureVal>(&data);
      log::debug!("AirPressure Info: {:?}", airpressure_info);
      if let Err(e) = app_handle.emit("airpressure_info", airpressure_info) {
        log::error!("Failed to emit airpressure info: {}", e);
      }
    })),
  )
  .await
}

#[tauri::command]
pub async fn stop_airpressure_info() -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(Arc::new(ble_transfer), "valve_info 0\r\n", 3, false, None).await
}
