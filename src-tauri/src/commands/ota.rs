use crate::{
  ota::{Ota, sample::SampleOta},
  transfer::ble::BleTransfer,
};
use std::sync::Arc;

#[tauri::command]
pub async fn start_valve_ota(app_handle: tauri::AppHandle) -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  let mut ota_impl = SampleOta::new(Arc::new(ble_transfer));

  ota_impl.start_ota(app_handle).await
}
