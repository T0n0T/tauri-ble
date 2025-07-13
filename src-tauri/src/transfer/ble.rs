use async_trait::async_trait;
use futures::TryFutureExt;
use tauri::Emitter;
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::Transfer;

const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe1-0000-1000-8000-00805f9b34fb");
const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe2-0000-1000-8000-00805f9b34fb");

const BLE_MTU: usize = 247 - 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BleDevice {
  name: String,
  address: String,
  isconnected: bool,
}

pub struct BleTransfer {
  handler: &'static tauri_plugin_blec::Handler,
  mac: String,
  mtu: usize,
}

impl BleTransfer {
  pub async fn new() -> Result<Self, String> {
    let handler =
      tauri_plugin_blec::get_handler().map_err(|e| format!("BLE handler unavailable: {:?}", e))?;
    let mac = handler
      .connected_device()
      .await
      .map_err(|e| format!("Failed to get connected device: {:?}", e))?
      .address;

    Ok(BleTransfer {
      handler,
      mac,
      mtu: BLE_MTU,
    })
  }
}

#[async_trait]
impl Transfer for BleTransfer {
  fn get_mtu(&self) -> usize {
    self.mtu
  }

  async fn activate(&self) -> Result<(), String> {
    self
      .handler
      .connect(
        self.mac.as_str(),
        tauri_plugin_blec::OnDisconnectHandler::None,
      )
      .await
      .map_err(|e| format!("BLE connection failed: {:?}", e))?;
    Ok(())
  }

  async fn deactivate(&self) -> Result<(), String> {
    self
      .handler
      .disconnect()
      .await
      .map_err(|e| format!("BLE deactivation failed: {:?}", e))
  }

  async fn is_actived(&self) -> Result<bool, String> {
    Ok(self.handler.is_connected())
  }

  async fn send(&self, data: &[u8]) -> Result<(), String> {
    if data.len() <= self.mtu {
      self
        .handler
        .send_data(WRITE_CHARACTERISTIC_UUID, data, tauri_plugin_blec::models::WriteType::WithResponse)
        .await
        .map_err(|e| format!("BLE send failed: {:?}", e))?;
      return Ok(());
    } else {
      return Err(format!(
        "Data size {} exceeds BLE MTU limit of {}",
        data.len(),
        self.mtu
      ));
    }
  }

  async fn read(&self) -> Result<Vec<u8>, String> {
    self
      .handler
      .recv_data(READ_CHARACTERISTIC_UUID)
      .await
      .map_err(|e| format!("BLE read failed: {:?}", e))
  }

  async fn subscribe(
    &self,
    callback: Arc<dyn Fn(Vec<u8>) + Send + Sync + 'static>,
  ) -> Result<(), String> {
    self
      .handler
      .subscribe(READ_CHARACTERISTIC_UUID, move |data| {
        callback(data);
      })
      .await
      .map_err(|e| format!("BLE subscribe failed: {:?}", e))
  }

  async fn unsubscribe(&self) -> Result<(), String> {
    self
      .handler
      .unsubscribe(READ_CHARACTERISTIC_UUID)
      .await
      .map_err(|e| format!("BLE unsubscribe failed: {:?}", e))
  }
}

#[tauri::command]
pub async fn connect(app_handle: tauri::AppHandle, device: BleDevice) -> Result<(), String> {
  let handler =
    tauri_plugin_blec::get_handler().map_err(|e| format!("BLE unavailable: {:?}", e))?;
  
  const MAX_RETRIES: usize = 3;
  const RETRY_DELAY_MS: u64 = 1000;
  
  let mut retry_count = 0;
  let mut last_error = String::new();
  
  while retry_count < MAX_RETRIES {
    let mut _device = device.clone();
    
    match handler
      .connect(
        &device.address,
        tauri_plugin_blec::OnDisconnectHandler::Async(Box::pin({
          let app_handle = app_handle.clone();
          let mut device = device.clone();
          async move {
            device.isconnected = false;
            let _ = app_handle.emit("ble_status", device);
          }
        })),
      )
      .await
    {
      Ok(_) => {
        _device.isconnected = true;
        let _ = app_handle.emit("ble_status", _device);
        return Ok(());
      }
      Err(e) => {
        retry_count += 1;
        last_error = format!("BLE connect {} failed: {:?}", device.address, e);
        
        if retry_count < MAX_RETRIES {
          tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
        }
      }
    }
  }
  
  Err(format!("{} after {} retries", last_error, MAX_RETRIES))
}

#[tauri::command]
pub async fn disconnect() -> Result<(), String> {
  tauri_plugin_blec::get_handler()
    .map_err(|e| format!("BLE unavailable: {:?}", e))?
    .disconnect()
    .map_err(|e| format!("BLE disconnect failed: {:?}", e))
    .await
}
