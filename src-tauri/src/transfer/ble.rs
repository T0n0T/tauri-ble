use async_trait::async_trait;
use std::sync::Arc;
use tauri_plugin_blec::models::WriteType;
use uuid::Uuid;

use super::Transfer;

const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("6e400003-b5a3-f393-e0a9-e50e24dcca9e");
const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("6e400002-b5a3-f393-e0a9-e50e24dcca9e");

pub struct BleTransfer;

impl BleTransfer {
  pub fn new() -> Self {
    BleTransfer
  }
}

#[async_trait]
impl Transfer for BleTransfer {
  async fn send_data(&self, data: &[u8]) -> Result<(), String> {
    tauri_plugin_blec::get_handler()
      .map_err(|e| format!("BLE handler unavailable: {:?}", e))?
      .send_data(WRITE_CHARACTERISTIC_UUID, data, WriteType::WithResponse)
      .await
      .map_err(|e| format!("BLE send failed: {:?}", e))?;
    Ok(())
  }

  async fn receive_data(&self) -> Result<Vec<u8>, String> {
    // BLE receive logic would go here. For now, returning an empty vector.
    // In a real scenario, this would involve listening for notifications/indications
    // on READ_CHARACTERISTIC_UUID.
    Ok(Vec::new())
  }

  async fn notify(
    &self,
    callback: Arc<dyn Fn(Vec<u8>) + Send + Sync + 'static>,
    enable: bool,
  ) -> Result<(), String> {
    if enable {
      tauri_plugin_blec::get_handler()
        .map_err(|e| format!("BLE handler unavailable: {:?}", e))?
        .subscribe(READ_CHARACTERISTIC_UUID, move |data| {
          callback(data);
        })
        .await
        .map_err(|e| format!("BLE subscribe failed: {:?}", e))
    } else {
      tauri_plugin_blec::get_handler()
        .map_err(|e| format!("BLE handler unavailable: {:?}", e))?
        .unsubscribe(READ_CHARACTERISTIC_UUID)
        .await
        .map_err(|e| format!("BLE unsubscribe failed: {:?}", e))
    }
  }
}
