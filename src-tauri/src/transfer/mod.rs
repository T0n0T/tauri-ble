use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Transfer {
  fn mtu(&self) -> usize;
  async fn send(&self, data: &[u8]) -> Result<(), String>;
  async fn read(&self) -> Result<Vec<u8>, String>;
  async fn subcribe(
    &self,
    callback: Arc<
      dyn (Fn(Vec<u8>) -> futures::future::BoxFuture<'static, ()>) + Send + Sync + 'static,
    >,
  ) -> Result<(), String>;
  async fn unsubscribe(&self) -> Result<(), String>;
}

pub mod ble;
