use crate::{ota::Ota, transfer::Transfer};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::time::Duration;
use std::{io::Read, sync::Arc};
use tauri::Emitter;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::{FsExt, OpenOptions};
use tokio::{sync::watch, time::Instant};

pub const DFU_PAGE_LEN: usize = 2048;
pub const DFU_PREAMBLE: [u8; 4] = [0xAA, 0x55, 0xAA, 0x55];
pub const DFU_ACK_PATTERN: u32 = 0x12345678; // 示例ACK模式，实际应根据MCU协议定义

// DFU状态枚举，对应Mermaid图
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DFUState {
  Start,
  SendPreamble,
  SendTotalBlocks,
  SendBlockHeader,
  SendBlockData,
  WaitVerify,
  WaitWrite,
  Complete,
  Fault,
}

// MCU响应状态枚举，对应bootdfu.c中的dfu_state
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum McuDfuState {
  Idle = 0,
  Prepare = 1,
  Header = 2,
  Data = 3,
  Verify = 4,
  Write = 5,
  Final = 6,
  Fault = 7,
}

impl std::convert::TryFrom<u8> for McuDfuState {
  type Error = String;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Idle),
      1 => Ok(Self::Prepare),
      2 => Ok(Self::Header),
      3 => Ok(Self::Data),
      4 => Ok(Self::Verify),
      5 => Ok(Self::Write),
      6 => Ok(Self::Final),
      7 => Ok(Self::Fault),
      _ => Ok(Self::Idle), // 默认返回Idle状态
    }
  }
}

// 对应bootdfu.c中的firmware_block_header
#[derive(Debug, Clone)]
pub struct FirmwareBlockHeader {
  pub signature: [u8; 64],
  pub block_size: u32,
}

impl FirmwareBlockHeader {
  pub fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&self.signature);
    bytes.extend_from_slice(&self.block_size.to_le_bytes());
    bytes
  }
}

pub struct SampleOta {
  transfer: Arc<dyn Transfer>,
  state: DFUState,
  mcu_state: McuDfuState,
  current_block_index: usize,
  current_chunk_index: usize,
  mcu_state_sender: watch::Sender<McuDfuState>,
  mcu_state_receiver: watch::Receiver<McuDfuState>,
}

impl SampleOta {
  pub fn new(transfer: Arc<dyn Transfer>) -> Self {
    let (tx, rx) = watch::channel(McuDfuState::Idle);
    SampleOta {
      transfer,
      state: DFUState::Start,
      mcu_state: McuDfuState::Idle,
      current_block_index: 0,
      current_chunk_index: 0,
      mcu_state_sender: tx,
      mcu_state_receiver: rx,
    }
  }

  pub async fn ota_process(
    &mut self,
    file_data: Arc<Vec<u8>>,
    total_blocks: usize,
  ) -> Result<(bool, bool), String> {
    let last_state = self.state;
    let mcu_target_state = *self.mcu_state_receiver.borrow();
    let mut progress_percentage_need_calculate = false;

    if mcu_target_state == self.mcu_state && self.mcu_state != McuDfuState::Idle {
      tokio::time::sleep(Duration::from_millis(10)).await;
      return Ok((false, false));
    }

    if mcu_target_state != McuDfuState::Idle
      && mcu_target_state != McuDfuState::Fault
      && mcu_target_state != self.mcu_state
    {
      self
        .transfer
        .send(&DFU_ACK_PATTERN.to_le_bytes())
        .await
        .map_err(|e| format!("OTA Ack failed: {:?}", e))?;
      //if error happened, retry next time will come here again
      self.mcu_state = mcu_target_state;
    }

    if self.mcu_state == McuDfuState::Fault {
      //mcu has been reboot, no way to retransfer
      self.state = DFUState::Fault;
    }
    match self.state {
      DFUState::Start => {
        info!("State: Start -> Sending OTA command");
        self
          .transfer
          .send("update\r\n".as_bytes())
          .await
          .map_err(|e| format!("OTA send failed: {:?}", e))?;
        self.state = DFUState::SendPreamble;
        tokio::time::sleep(Duration::from_secs(1)).await;
      }
      DFUState::SendPreamble => {
        if self.mcu_state == McuDfuState::Idle {
          self
            .transfer
            .send(&DFU_PREAMBLE)
            .await
            .map_err(|e| format!("OTA send failed: {:?}", e))?;
          info!("State: SendPreamble -> Preamble sent, waiting for MCU response");
          self.state = DFUState::SendTotalBlocks;
        }
      }
      DFUState::SendTotalBlocks => {
        if self.mcu_state == McuDfuState::Prepare {
          info!("State: SendTotalBlocks -> MCU Ready for Total Blocks");
          let total_blocks_bytes = (total_blocks as u32).to_le_bytes();
          self
            .transfer
            .send(&total_blocks_bytes)
            .await
            .map_err(|e| format!("OTA send failed: {:?}", e))?;
          info!(
            "State: SendTotalBlocks -> Sending Total Blocks: {}",
            total_blocks
          );
          self.state = DFUState::SendBlockHeader;
        }
      }
      DFUState::SendBlockHeader => {
        if self.mcu_state == McuDfuState::Header {
          info!("State: SendBlockHeader -> MCU Ready for Block Header");
          // 模拟块头数据，实际应从固件文件中解析
          let block_header = FirmwareBlockHeader {
            signature: [0u8; 64], // 示例签名
            block_size: (file_data.len() - self.current_block_index * DFU_PAGE_LEN)
              .min(DFU_PAGE_LEN) as u32, // 实际块大小
          };
          let block_header_bytes = block_header.to_bytes();
          self
            .transfer
            .send(&block_header_bytes)
            .await
            .map_err(|e| format!("OTA send failed: {:?}", e))?;
          info!(
            "State: SendBlockHeader -> Sending Block {} Header",
            self.current_block_index
          );
          self.state = DFUState::SendBlockData;
        }
      }
      DFUState::SendBlockData => {
        if self.mcu_state == McuDfuState::Data {
          info!("State: SendBlockData -> MCU Ready for Block Data");
          let start = self.current_block_index * DFU_PAGE_LEN;
          let end = (start + DFU_PAGE_LEN).min(file_data.len());
          let block_data = &file_data[start..end];

          let chunks = block_data
            .chunks(self.transfer.get_mtu())
            .collect::<Vec<_>>();
          info!(
            "Block {} Data Size: {}, Chunks: {}",
            self.current_block_index,
            block_data.len(),
            chunks.len()
          );
          for chunk in &chunks[self.current_chunk_index..] {
            self.transfer.send(*chunk).await.map_err(|e| {
              format!(
                "OTA chunk {} send failed: {:?}",
                self.current_chunk_index, e
              )
            })?;
            info!(
              "Sent chunk {} of size: {}",
              self.current_chunk_index,
              chunk.len()
            );
            self.current_chunk_index += 1;
          }
          self.current_chunk_index = 0;
          info!(
            "State: SEND_BLOCK_DATA -> Sending Block {} Data",
            self.current_block_index
          );
          self.state = DFUState::WaitVerify;
        }
      }
      DFUState::WaitVerify => {
        if self.mcu_state == McuDfuState::Verify {
          info!(
            "State: WAIT_VERIFY -> MCU Verify Block {}",
            self.current_block_index
          );
          self.state = DFUState::WaitWrite;
        }
      }
      DFUState::WaitWrite => {
        if self.mcu_state == McuDfuState::Write {
          info!(
            "State: WAIT_WRITE -> MCU Write Block {}",
            self.current_block_index
          );
          self.current_block_index += 1;
          if self.current_block_index >= total_blocks {
            self.state = DFUState::Complete;
          } else {
            self.state = DFUState::SendBlockHeader; // 准备发送下一个块的头部
            progress_percentage_need_calculate = true;
          }
        }
      }
      DFUState::Complete => {
        if self.mcu_state == McuDfuState::Final {
          progress_percentage_need_calculate = true;
        }
      }
      DFUState::Fault => {
        return Err("OTA process failed".to_string());
      }
    }
    return Ok((
      self.state != last_state, // true if state changed, false otherwise
      progress_percentage_need_calculate,
    ));
  }
}

#[async_trait]
impl Ota for SampleOta {
  async fn start_ota(&mut self, app_handle: tauri::AppHandle) -> Result<(), String> {
    // Apps can fully manage entries within this directory with std::fs.
    let file_path = app_handle.dialog().file().blocking_pick_file().unwrap();
    let mut opt = OpenOptions::new();
    opt.read(true);
    debug!("Starting OTA for file: {:?}", file_path);
    let file_data = Arc::new(
      app_handle
        .fs()
        .open(file_path, opt)
        .map_err(|e| format!("Failed to read file: {}", e))?
        .bytes()
        .collect::<Result<Vec<u8>, std::io::Error>>()
        .map_err(|e| format!("Failed to read file bytes: {}", e))?,
    );

    let mcu_state_sender_clone = self.mcu_state_sender.clone();
    let total_blocks = (file_data.len() + DFU_PAGE_LEN - 1) / DFU_PAGE_LEN;

    let subscribe_callback = Arc::new(move |data: Vec<u8>| {
      if let Some(&state_byte) = data.first() {
        if let Ok(mcu_state) = McuDfuState::try_from(state_byte) {
          let _ = mcu_state_sender_clone.send(mcu_state);
          debug!("Received MCU state: {:?}", mcu_state);
        } else {
          error!("Failed to parse MCU state byte: {}", state_byte);
        }
      } else {
        warn!("Received empty data from MCU notify.");
      }
    });

    self.transfer.unsubscribe().await.ok();

    self
      .transfer
      .subscribe(subscribe_callback.clone())
      .await
      .map_err(|e| format!("Failed to subscribe to OTA: {}", e))?;

    debug!("Notify callback set for transfer");

    let mut last_state_change_time = Instant::now();
    let mut retry = 3;
    let ota_result = loop {
      let ota_process_result = self.ota_process(file_data.clone(), total_blocks).await;

      match ota_process_result {
        Ok((state_changed, progress_needs_calculate)) => {
          if self.state == DFUState::Fault {
            error!("OTA process failed due to DFUState::Fault.");
            app_handle
              .emit("ota_error", "OTA process failed by MCU fault")
              .map_err(|e| format!("Failed to emit OTA error: {}", e))?;
            break Err("OTA process failed".to_string());
          }
          if state_changed {
            last_state_change_time = Instant::now();
          } else {
            // 检查是否超过60秒没有状态更改
            if last_state_change_time.elapsed() > Duration::from_secs(60) {
              break Err(format!("OTA process Timeout"));
            }
          }

          if progress_needs_calculate {
            // 计算进度条
            let progress_percentage =
              ((self.current_block_index as f64 / total_blocks as f64) * 100.0) as u32;
            app_handle
              .emit("ota_progress", progress_percentage)
              .map_err(|e| format!("Failed to emit OTA progress: {}", e))?;
          }

          if self.state == DFUState::Complete && self.mcu_state == McuDfuState::Final {
            info!("OTA process completed successfully.");
            break Ok(());
          }
        }
        Err(e) => {
          error!("OTA process error: {}", e);
          if retry > 0 {
            retry -= 1;
            warn!("Retrying OTA process, attempts left: {}", retry);
            if self.transfer.is_actived().await.ok().unwrap() {
              self.transfer.unsubscribe().await.ok();
              warn!("Unsubscribed from OTA, retrying...");
            }
            self
              .transfer
              .deactivate()
              .await
              .map_err(|e| format!("Failed to deactivate transfer: {}", e))?;
            self
              .transfer
              .activate()
              .await
              .map_err(|e| format!("Failed to activate transfer: {}", e))?;
            warn!("Re-activating transfer for OTA...");
            self
              .transfer
              .subscribe(subscribe_callback.clone())
              .await
              .map_err(|e| format!("Failed to re-subscribe to OTA: {}", e))?;
            warn!("Re-subscribed to OTA, retrying...");
            continue; // 继续循环重试
          } else {
            error!("All retries exhausted, OTA process failed.");
            app_handle
              .emit("ota_error", "OTA process failed by retries exhausted")
              .map_err(|e| format!("Failed to emit OTA error: {}", e))?;
            break Err(e);
          }
        }
      }
      tokio::time::sleep(Duration::from_millis(5)).await; // 短暂延迟，避免CPU占用过高
    };

    // 无论OTA过程成功或失败，都尝试停止通知
    if let Err(e) = self.transfer.unsubscribe().await {
      warn!("Failed to unsubscribe OTA: {}", e);
    }
    ota_result
  }
}
