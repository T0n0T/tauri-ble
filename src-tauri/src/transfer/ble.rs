use async_trait::async_trait;
use std::sync::Arc;
use tauri_plugin_blec::models::WriteType;
use uuid::Uuid;

use super::Transfer;

const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe1-0000-1000-8000-00805f9b34fb");
const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe2-0000-1000-8000-00805f9b34fb");

const BLE_MTU: usize = 247 - 3;

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
    self.handler
      .connect(self.mac.as_str(), tauri_plugin_blec::OnDisconnectHandler::None)
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
        .send_data(WRITE_CHARACTERISTIC_UUID, data, WriteType::WithResponse)
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
    tauri_plugin_blec::get_handler()
      .map_err(|e| format!("BLE handler unavailable: {:?}", e))?
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
