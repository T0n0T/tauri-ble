use super::{
  RealtimeInfoKind, ValveVal, do_request_response, send_and_unsubscribe, start_realtime_info,
  stop_realtime_info,
};
use crate::transfer::ble::BleTransfer;
use std::sync::Arc;
use tauri::Emitter;

#[tauri::command]
pub async fn start_valve_info(app_handle: tauri::AppHandle) -> Result<(), String> {
  if !start_realtime_info(RealtimeInfoKind::Valve).await? {
    log::debug!("valve realtime info is already active; ignoring duplicate start");
    return Ok(());
  }

  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e));
  let ble_transfer = match ble_transfer {
    Ok(transfer) => transfer,
    Err(error) => {
      stop_realtime_info(RealtimeInfoKind::Valve).await;
      return Err(error);
    }
  };

  let result = do_request_response(
    Arc::new(ble_transfer),
    "valve_info 1\r\n",
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
      if let Err(e) = app_handle.emit("valve_info", valve_info) {
        log::error!("Failed to emit valve info: {}", e);
      }
    })),
  )
  .await;

  if result.is_err() {
    stop_realtime_info(RealtimeInfoKind::Valve).await;
  }
  result
}

#[tauri::command]
pub async fn stop_valve_info() -> Result<(), String> {
  let result = match BleTransfer::new().await {
    Ok(ble_transfer) => send_and_unsubscribe(Arc::new(ble_transfer), "valve_info 0\r\n").await,
    Err(error) => Err(format!("Create BLE Transfer failed: {}", error)),
  };
  stop_realtime_info(RealtimeInfoKind::Valve).await;
  result
}
