use super::{ChannelConfig, do_request_response};
use crate::transfer::ble::BleTransfer;
use std::sync::Arc;
use tauri::Emitter;

#[tauri::command]
pub async fn channel_configure(config: ChannelConfig) -> Result<(), String> {
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
pub async fn channel_readconfig(app_handle: tauri::AppHandle) -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  do_request_response(
    Arc::new(ble_transfer),
    "config_read\r\n",
    3,
    false,
    Some(Arc::new(
      move |data: Vec<u8>| match serde_json::from_slice::<ChannelConfig>(&data) {
        Ok(channel_config) => {
          log::debug!("Channel config: {:?}", channel_config);
          if let Err(e) = app_handle.emit("channel_config", channel_config) {
            log::error!("Failed to emit channel config: {}", e);
          }
        }
        Err(e) => {
          log::error!("Failed to parse valve config: {}", e);
        }
      },
    )),
  )
  .await
}

#[tauri::command]
pub async fn channel_refactory() -> Result<(), String> {
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