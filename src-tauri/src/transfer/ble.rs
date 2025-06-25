use async_trait::async_trait;
use std::sync::Arc;
use tauri_plugin_blec::models::WriteType;
use uuid::Uuid;

use super::Transfer;

const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe1-0000-1000-8000-00805f9b34fb");
const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe2-0000-1000-8000-00805f9b34fb");

pub struct BleTransfer;

const BLE_MTU: usize = 247 - 3;

impl BleTransfer {
  pub fn new() -> Self {
    BleTransfer
  }
}

#[async_trait]
impl Transfer for BleTransfer {
  async fn send_data(&self, data: &[u8]) -> Result<(), String> {
    let handler =
      tauri_plugin_blec::get_handler().map_err(|e| format!("BLE handler unavailable: {:?}", e))?;

    // 如果数据小于等于 MTU，直接发送
    if data.len() <= BLE_MTU {
      handler
        .send_data(WRITE_CHARACTERISTIC_UUID, data, WriteType::WithResponse)
        .await
        .map_err(|e| format!("BLE send failed: {:?}", e))?;
      return Ok(());
    }

    // 数据超过 MTU，分包发送
    for chunk in data.chunks(BLE_MTU) {
      handler
        .send_data(WRITE_CHARACTERISTIC_UUID, chunk, WriteType::WithResponse)
        .await
        .map_err(|e| format!("BLE chunk send failed: {:?}", e))?;
    }

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
