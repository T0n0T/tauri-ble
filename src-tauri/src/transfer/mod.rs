use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Transfer: Send + Sync {
  fn get_mtu(&self) -> usize;
  async fn activate(&self) -> Result<(), String>;
  async fn deactivate(&self) -> Result<(), String>;
  async fn is_actived(&self) -> Result<bool, String>;
  async fn send(&self, data: &[u8]) -> Result<(), String>;
  async fn read(&self) -> Result<Vec<u8>, String>;
  async fn subscribe(
    &self,
    callback: Arc<dyn Fn(Vec<u8>) + Send + Sync + 'static>,
  ) -> Result<(), String>;
  async fn unsubscribe(&self) -> Result<(), String>;
}

pub mod ble;
