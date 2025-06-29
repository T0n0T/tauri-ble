pub mod sample;

use async_trait::async_trait;
#[async_trait]
pub trait Ota {
  async fn start_ota(
    &mut self,
    app_handle: tauri::AppHandle,
  ) -> Result<(), String>;
}
