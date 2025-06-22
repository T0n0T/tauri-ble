pub mod sample;

use async_trait::async_trait;
use crate::transfer::Transfer;

#[async_trait]
pub trait Ota {
    async fn start_ota<T: Transfer + Send + Sync>(&self, app_handle: tauri::AppHandle, file_path: String, transfer: T) -> Result<(), String>;
}
