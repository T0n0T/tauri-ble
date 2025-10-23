use crate::transfer::Transfer;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{Duration, timeout};

pub mod ota;
pub mod ping;
pub mod reboot;
pub mod valve_config;
pub mod valve_info;
pub mod channel_config;
pub mod airpressure_config;
pub mod airpressure_info;

const CMD_OK: u16 = 0xcafe;
const CMD_ERR: u16 = 0xdead;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValveConfig {
  model: String,
  tick: u32,
  dir: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Serialize)]
pub struct ValveVal {
  total_ticks: i32,
  current_status: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
  model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirPressureConfig {
  model: String,
  pressure: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Serialize)]
pub struct AirPressureVal {
  current_pressure: f32,
}

async fn do_request_response(
  transfer: Arc<dyn Transfer>,
  command_str: &str,
  time_wait: u64,
  keep_subscribe: bool,
  callback: Option<Arc<dyn Fn(Vec<u8>) + Send + Sync + 'static>>,
) -> Result<(), String> {
  let (tx, mut rx) = mpsc::channel::<u16>(1);
  transfer.unsubscribe().await.ok();
  transfer
    .subscribe(Arc::new(move |data: Vec<u8>| {
      match data.as_slice() {
        [b0, b1, rest @ ..] => {
          // 捕获剩余数据
          let response_code = u16::from_le_bytes([*b0, *b1]);
          if response_code == CMD_OK || response_code == CMD_ERR {
            let _ = tx.blocking_send(response_code);
            // 如果有剩余数据，并且回调存在，则调用回调
            if !rest.is_empty() {
              if let Some(cb) = callback.clone() {
                cb(rest.to_vec()); // 将剩余数据传递给回调
              }
            }
          } else if let Some(cb) = callback.clone() {
            cb(data); // 如果不是CMD_OK或CMD_ERR，则传递整个数据
          }
        }
        _ => {
          if let Some(cb) = callback.clone() {
            cb(data);
          }
        }
      }
    }))
    .await
    .map_err(|e| format!("Failed to subscirbe: {}", e))?;

  if let Err(e) = transfer.send(command_str.as_bytes()).await {
    if !keep_subscribe {
      transfer.unsubscribe().await.ok();
    }
    return Err(format!("Failed to send {}: {}", command_str, e));
  }

  let result = match timeout(Duration::from_secs(time_wait), rx.recv()).await {
    Ok(Some(response_code)) => {
      if response_code == CMD_OK {
        log::info!("Successfully execuate command:{}", command_str);
        Ok(())
      } else if response_code == CMD_ERR {
        log::error!("Fail to execuate command:{}, receive CMD_ERR", command_str);
        Err(format!("Received CMD_ERR"))
      } else {
        // This case should ideally be handled by the callback, but as a fallback
        // if it somehow reaches here, we'll treat it as an unknown response.
        Err(format!("Received unknown response: {:?}", response_code))
      }
    }
    Ok(None) => {
      log::error!(
        "Fail to execuate command:{}, without any response",
        command_str
      );
      Err(format!(
        "Channel closed before receiving response after {} seconds",
        time_wait
      ))
    }
    Err(_) => {
      log::error!(
        "Fail to execuate command:{}, timeout {} without any response",
        time_wait,
        command_str
      );
      Err(format!("Response timeout in {} seconds", time_wait))
    }
  };

  if !keep_subscribe {
    transfer
      .unsubscribe()
      .await
      .map_err(|e| format!("Failed to unsubscribe: {}", e))?;
  }

  result
}
