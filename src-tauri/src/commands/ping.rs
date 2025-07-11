use crate::transfer::ble::BleTransfer;
use super::do_request_response;
use std::sync::Arc;

#[tauri::command]
pub async fn ping() -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(Arc::new(ble_transfer), "ping\r\n", 3, false, None).await
}
