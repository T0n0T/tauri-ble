// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

mod commands;
mod ota;
mod transfer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::new().build())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_blec::init())
    .plugin(
      tauri_plugin_log::Builder::new()
        .level(log::LevelFilter::Info)
        .level_for("tauri_bluetooth_tool_lib", LevelFilter::Trace)
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
        // .with_colors(
        //   ColoredLevelConfig::new()
        //     .trace(Color::Blue)
        //     .debug(Color::Magenta)
        //     .info(Color::Green),
        // )
        .build(),
    )
    .invoke_handler(tauri::generate_handler![
      commands::ota::start_valve_ota,
      commands::ping::ping,
      commands::reboot::reboot_valve,
      commands::valve_config::valve_configure,
      commands::valve_config::valve_refactory,
      commands::valve_config::valve_tunning_start,
      commands::valve_config::valve_tunning_stop,
      commands::valve_info::start_valve_info,
      commands::valve_info::stop_valve_info,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
