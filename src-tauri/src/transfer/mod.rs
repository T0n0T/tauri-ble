use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Transfer {
    async fn send_data(&self, data: &[u8]) -> Result<(), String>;
    async fn receive_data(&self) -> Result<Vec<u8>, String>;
    async fn notify(&self, callback: Arc<dyn Fn(Vec<u8>) + Send + Sync + 'static>) -> Result<(), String>;
}

pub mod ble;