use crate::ota::Ota;
use crate::transfer::Transfer;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tauri::Emitter;
use tokio::{fs, sync::watch, time::Instant};

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
  FAULT = 7,
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
      7 => Ok(Self::FAULT),
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
  mcu_state_sender: watch::Sender<McuDfuState>,
  mcu_state_receiver: watch::Receiver<McuDfuState>,
}

impl SampleOta {
  pub fn new() -> Self {
    let (tx, rx) = watch::channel(McuDfuState::Idle);
    SampleOta {
      mcu_state_sender: tx,
      mcu_state_receiver: rx,
    }
  }
}

#[async_trait]
impl Ota for SampleOta {
  async fn start_ota<T: Transfer + Send + Sync>(
    &self,
    app_handle: tauri::AppHandle,
    file_path: String,
    transfer: T,
  ) -> Result<(), String> {
    println!("Starting OTA for file: {}", file_path);

    let file_data = fs::read(&file_path)
      .await
      .map_err(|e| format!("Failed to read file: {}", e))?;

    let mcu_state_sender_clone = self.mcu_state_sender.clone();
    let mcu_state_receiver_clone = self.mcu_state_receiver.clone();

    // 设置notify回调
    transfer
      .notify(
        Arc::new(move |data: Vec<u8>| {
          if let Some(&state_byte) = data.first() {
            if let Ok(mcu_state) = McuDfuState::try_from(state_byte) {
              let _ = mcu_state_sender_clone.send(mcu_state);
              println!("Received MCU state: {:?}", mcu_state);
            } else {
              println!("Failed to parse MCU state byte: {}", state_byte);
            }
          } else {
            println!("Received empty data from MCU notify.");
          }
        }),
        true,
      )
      .await?;

    let mut current_state = DFUState::Start;
    let mut current_block_index = 0;
    let mut last_state_change = Instant::now();
    let mut last_mcu_response_state = McuDfuState::Idle; // 初始MCU状态
    let total_blocks = (file_data.len() + DFU_PAGE_LEN - 1) / DFU_PAGE_LEN;

    loop {
      let mcu_response_state = *mcu_state_receiver_clone.borrow();

      // if last_state_change.elapsed() > Duration::from_secs(10) {
      //   println!("OTA process timed out in state: {:?}", current_state);
      //   current_state = DFUState::Fault;
      // }

      if current_state != DFUState::Start
        && current_state != DFUState::Complete
        && current_state != DFUState::Fault
      {
        if mcu_response_state == last_mcu_response_state && mcu_response_state != McuDfuState::Idle
        {
          tokio::time::sleep(Duration::from_millis(10)).await;
          continue;
        }
        last_mcu_response_state = mcu_response_state;
      }
      if mcu_response_state == McuDfuState::FAULT {
        current_state = DFUState::Fault;
      }
      match current_state {
        DFUState::Start => {
          println!("State: Start -> Sending OTA command");
          transfer.send_data("update\r\n".as_bytes()).await?;
          current_state = DFUState::SendPreamble;
          tokio::time::sleep(Duration::from_secs(1)).await;
          last_state_change = Instant::now();
        }
        DFUState::SendPreamble => {
          if mcu_response_state == McuDfuState::Idle {
            transfer.send_data(&DFU_PREAMBLE).await?;
            println!("State: SendPreamble -> Preamble sent, waiting for MCU response");
            current_state = DFUState::SendTotalBlocks;
            last_state_change = Instant::now();
          }
        }
        DFUState::SendTotalBlocks => {
          if mcu_response_state == McuDfuState::Prepare {
            transfer.send_data(&DFU_ACK_PATTERN.to_le_bytes()).await?;
            println!("State: SendTotalBlocks -> MCU Ready for Total Blocks");
            let total_blocks_bytes = (total_blocks as u32).to_le_bytes();
            transfer.send_data(&total_blocks_bytes).await?;
            println!(
              "State: SendTotalBlocks -> Sending Total Blocks: {}",
              total_blocks
            );
            current_state = DFUState::SendBlockHeader;
            last_state_change = Instant::now();
          }
        }
        DFUState::SendBlockHeader => {
          if mcu_response_state == McuDfuState::Header {
            transfer.send_data(&DFU_ACK_PATTERN.to_le_bytes()).await?;
            println!("State: SendBlockHeader -> MCU Ready for Block Header");
            // 模拟块头数据，实际应从固件文件中解析
            let block_header = FirmwareBlockHeader {
              signature: [0u8; 64], // 示例签名
              block_size: (file_data.len() - current_block_index * DFU_PAGE_LEN).min(DFU_PAGE_LEN)
                as u32, // 实际块大小
            };
            let block_header_bytes = block_header.to_bytes();
            transfer.send_data(&block_header_bytes).await?;
            println!(
              "State: SendBlockHeader -> Sending Block {} Header",
              current_block_index
            );
            current_state = DFUState::SendBlockData;
            last_state_change = Instant::now();
          }
        }
        DFUState::SendBlockData => {
          if mcu_response_state == McuDfuState::Data {
            transfer.send_data(&DFU_ACK_PATTERN.to_le_bytes()).await?;
            println!("State: SendBlockData -> MCU Ready for Block Data");
            let start = current_block_index * DFU_PAGE_LEN;
            let end = (start + DFU_PAGE_LEN).min(file_data.len());
            let block_data = &file_data[start..end];
            transfer.send_data(block_data).await?;
            println!(
              "State: SEND_BLOCK_DATA -> Sending Block {} Data",
              current_block_index
            );
            current_state = DFUState::WaitVerify;
            last_state_change = Instant::now();
          }
        }
        DFUState::WaitVerify => {
          if mcu_response_state == McuDfuState::Verify {
            transfer.send_data(&DFU_ACK_PATTERN.to_le_bytes()).await?;
            println!(
              "State: WAIT_VERIFY -> MCU Verify Block {}",
              current_block_index
            );
            last_state_change = Instant::now();
          } else if mcu_response_state == McuDfuState::Write {
            transfer.send_data(&DFU_ACK_PATTERN.to_le_bytes()).await?;
            if current_block_index < total_blocks - 1 {
              current_block_index += 1;
              let progress_percentage =
                ((current_block_index as f64 / total_blocks as f64) * 100.0) as u32;
              app_handle
                .emit("ota_progress", progress_percentage)
                .unwrap();
              current_state = DFUState::SendBlockHeader;
              last_state_change = Instant::now();
            }
          } else if mcu_response_state == McuDfuState::Final {
            transfer.send_data(&DFU_ACK_PATTERN.to_le_bytes()).await?;
            current_state = DFUState::Complete;
            last_state_change = Instant::now();
          }
        }
        DFUState::Complete => {
          println!("OTA process completed successfully for file: {}", file_path);
          app_handle.emit("ota_progress", 100).unwrap(); // 通知前端完成
          break;
        }
        DFUState::Fault => {
          println!("OTA process failed for file: {}", file_path);
          app_handle.emit("ota_progress", 0).unwrap(); // 通知前端失败
          app_handle.emit("ota_error", "OTA process failed").unwrap(); // 通知前端OTA错误
          break;
        }
      }
      tokio::time::sleep(Duration::from_millis(20)).await; // 避免忙循环，等待MCU响应
    }
    transfer.notify(Arc::new(|_| {}), false).await?; // 停止通知
    Ok(())
  }
}
