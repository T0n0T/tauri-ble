use async_trait::async_trait;
use std::sync::Arc;
use tauri_plugin_blec::models::WriteType;
use uuid::Uuid;

use super::Transfer;

const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe1-0000-1000-8000-00805f9b34fb");
const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe2-0000-1000-8000-00805f9b34fb");

const BLE_MTU: usize = 247 - 3;

pub struct BleTransfer<'a> {
  handler: &'a tauri_plugin_blec::Handler,
  mtu: usize,
}

impl<'a> BleTransfer<'a> {
  pub fn new() -> Self {
    let handler = tauri_plugin_blec::get_handler().expect("BLE handler unavailable");
    BleTransfer {
      handler,
      mtu: BLE_MTU,
    }
  }
}

#[async_trait]
impl<'a> Transfer for BleTransfer<'a> {
  fn mtu(&self) -> usize {
    self.mtu
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

  async fn subcribe(
    &self,
    callback: Arc<
      dyn (Fn(Vec<u8>) -> futures::future::BoxFuture<'static, ()>) + Send + Sync + 'static,
    >,
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
