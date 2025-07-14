use tauri::Manager;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn openlog(app_handle: tauri::AppHandle) -> Result<(), String> {
  let logs_path = app_handle
    .path()
    .app_log_dir()
    .unwrap()
    .as_path()
    .to_string_lossy()
    .into_owned();
  let logs_path_bak = logs_path.clone();
  app_handle.opener()
    .open_path(logs_path, None::<&str>)
    .map_err(|e| format!("open logs path {} error {:?}", logs_path_bak, e))
}
