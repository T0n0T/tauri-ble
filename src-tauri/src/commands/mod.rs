use crate::transfer::{Transfer, ble::BleTransfer};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{Duration, timeout};
use serde::{Deserialize, Serialize};
use bytemuck::{Pod, Zeroable};

pub mod ota;
pub mod ping;
pub mod reboot;
pub mod valve_config;
pub mod valve_info;

const CMD_OK: u16 = 0xcafe;
const CMD_ERR: u16 = 0xdead;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValveConfig {
  model: String,
  count: u32,
  dir: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Serialize)]
pub struct ValveVal {
  total_ticks: i32,
  current_status: u32,
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
    .subscribe(Arc::new(move |data: Vec<u8>| match data.as_slice() {
      [b0, b1, ..] => {
        let response_code = u16::from_le_bytes([*b0, *b1]);
        if response_code == CMD_OK || response_code == CMD_ERR {
          let _ = tx.blocking_send(response_code);
        } else if let Some(cb) = callback.clone() {
          cb(data);
        }
      }
      _ => {
        if let Some(cb) = callback.clone() {
          cb(data);
        }
      }
    }))
    .await
    .map_err(|e| format!("Failed to start valve info: {}", e))?;

  if let Err(e) = transfer.send(command_str.as_bytes()).await {
    if !keep_subscribe {
      transfer.unsubscribe().await.ok();
    }
    return Err(format!("Failed to send {}: {}", command_str, e));
  }

  let result = match timeout(Duration::from_secs(time_wait), rx.recv()).await {
    Ok(Some(response_code)) => {
      if response_code == CMD_OK {
        Ok(())
      } else if response_code == CMD_ERR {
        Err(format!("Received CMD_ERR"))
      } else {
        // This case should ideally be handled by the callback, but as a fallback
        // if it somehow reaches here, we'll treat it as an unknown response.
        Err(format!("Received unknown response: {:?}", response_code))
      }
    }
    Ok(None) => Err(format!(
      "Channel closed before receiving response after {} seconds",
      time_wait
    )),
    Err(_) => Err(format!("Response timeout in {} seconds", time_wait)),
  };

  if !keep_subscribe {
    transfer
      .unsubscribe()
      .await
      .map_err(|e| format!("Failed to unsubscribe: {}", e))?;
  }

  result
}
