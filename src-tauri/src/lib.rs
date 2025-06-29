// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
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

  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  ble_transfer.send(payload.as_bytes()).await
}

#[tauri::command]
async fn start_valve_info(app_handle: tauri::AppHandle) -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  ble_transfer
    .subscribe(Arc::new(move |data: Vec<u8>| {
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
    }))
    .await
    .map_err(|e| format!("Failed to start valve info: {}", e))?;

  ble_transfer.send("valve_info 1\r\n".as_bytes()).await
}

#[tauri::command]
async fn stop_valve_info() -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  ble_transfer.unsubscribe().await?;
  ble_transfer.send("valve_info 0\r\n".as_bytes()).await
}

#[tauri::command]
async fn reboot_valve() -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  ble_transfer.send("reboot\r\n".as_bytes()).await
  // ble_transfer.unsubscribe().await.ok();
  // let subscribe_callback = Arc::new(move |data: Vec<u8>| {
  //   println!("response: {:?}", data);
  // });
  // ble_transfer
  // .subscribe(subscribe_callback.clone())
  // .await
  // .map_err(|e| format!("Failed to subscribe: {}", e))?;
  // println!("Subscribed");
  // ble_transfer.unsubscribe().await.ok();
  // println!("Unsubscribed...");
  // ble_transfer
  //   .deactivate()
  //   .await
  //   .map_err(|e| format!("Failed to deactivate transfer: {}", e))?;
  // ble_transfer
  //   .activate()
  //   .await
  //   .map_err(|e| format!("Failed to activate transfer: {}", e))?;
  // println!("Re-activating transfer for OTA...");
  // ble_transfer
  //   .subscribe(subscribe_callback.clone())
  //   .await
  //   .map_err(|e| format!("Failed to re-subscribe to OTA: {}", e))?;
  // println!("Re-subscribed to OTA, finish...");
  // Ok(())
}

#[tauri::command]
async fn start_valve_ota(app_handle: tauri::AppHandle) -> Result<(), String> {
  let ble_transfer = BleTransfer::new()
    .await
    .map_err(|e| format!("Create BLE Transfer failed: {}", e))?;
  let mut ota_impl = SampleOta::new(Arc::new(ble_transfer));

  ota_impl.start_ota(app_handle).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::new().build())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_blec::init())
    .plugin(
      tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .level_for("tauri_bluetooth_tool_lib", LevelFilter::Trace)
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
        // .with_colors(
        //   ColoredLevelConfig::new()
        //     .trace(Color::Blue)
        //     .debug(Color::Magenta)
        //     .info(Color::Green),
        // )
        .build(),
    )
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
