use super::{
  AirPressureVal, RealtimeInfoKind, do_request_response, send_and_unsubscribe, start_realtime_info,
  stop_realtime_info,
};
use crate::transfer::ble::BleTransfer;
use std::sync::Arc;
use tauri::Emitter;

#[tauri::command]
pub async fn start_airpressure_info(app_handle: tauri::AppHandle) -> Result<(), String> {
  if !start_realtime_info(RealtimeInfoKind::AirPressure).await? {
    log::debug!("airpressure realtime info is already active; ignoring duplicate start");
    return Ok(());
  }

  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e));
  let ble_transfer = match ble_transfer {
    Ok(transfer) => transfer,
    Err(error) => {
      stop_realtime_info(RealtimeInfoKind::AirPressure).await;
      return Err(error);
    }
  };

  let result = do_request_response(
    Arc::new(ble_transfer),
    "airpressure_info 1\r\n",
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
  .await;

  if result.is_err() {
    stop_realtime_info(RealtimeInfoKind::AirPressure).await;
  }
  result
}

#[tauri::command]
pub async fn stop_airpressure_info() -> Result<(), String> {
  let result = match BleTransfer::new().await {
    Ok(ble_transfer) => {
      send_and_unsubscribe(Arc::new(ble_transfer), "airpressure_info 0\r\n").await
    }
    Err(error) => Err(format!("Create BLE Transfer failed: {}", error)),
  };
  stop_realtime_info(RealtimeInfoKind::AirPressure).await;
  result
}
