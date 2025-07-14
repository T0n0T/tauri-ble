// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use tauri::Manager;
use std::{path::PathBuf, vec};

mod commands;
mod ota;
mod transfer;

#[cfg(target_os = "android")]
fn default_log_targets() -> Vec<tauri_plugin_log::Target> {
  let log_dir = PathBuf::from("/sdcard/Android/data/com.bluetooth.tool/logs");  
  vec![
    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Folder {
      path: log_dir,
      file_name: None,
    }),
  ]
}

#[cfg(not(target_os = "android"))]
fn default_log_targets() -> Vec<tauri_plugin_log::Target> {
  vec![
    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { file_name: None }),
  ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      transfer::ble::connect,
      transfer::ble::disconnect,
      commands::ota::start_valve_ota,
      commands::ping::ping,
      commands::reboot::reboot_valve,
      commands::valve_config::valve_configure,
      commands::valve_config::valve_readconfig,
      commands::valve_config::valve_refactory,
      commands::valve_config::valve_tuning_start,
      commands::valve_config::valve_tuning_stop,
      commands::valve_info::start_valve_info,
      commands::valve_info::stop_valve_info,
    ])
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_blec::init())
    .plugin(
      tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .level_for("tauri_bluetooth_tool_lib", LevelFilter::Trace)
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
        .targets(default_log_targets())
        // .with_colors(
        //   ColoredLevelConfig::new()
        //     .trace(Color::Blue)
        //     .debug(Color::Magenta)
        //     .info(Color::Green),
        // )
        .build(),
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
