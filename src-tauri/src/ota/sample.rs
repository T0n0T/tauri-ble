use crate::ota::Ota;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tauri::Emitter;
use tauri_plugin_blec::get_handler;
use tauri_plugin_blec::models::WriteType;
use tokio::{fs, sync::watch, time::Instant};
use uuid::Uuid;

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
  async fn start_ota(&self, app_handle: tauri::AppHandle, file_path: String) -> Result<(), String> {
    let file_data = fs::read(&file_path)
      .await
      .map_err(|e| format!("Failed to read file: {}", e))?;

    println!("Starting OTA for file: {}", file_path);

    let mcu_state_sender_clone = self.mcu_state_sender.clone();
    let mcu_state_receiver_clone = self.mcu_state_receiver.clone();

    const READ_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe1-0000-1000-8000-00805f9b34fb");
    const WRITE_CHARACTERISTIC_UUID: Uuid = uuid::uuid!("0000ffe2-0000-1000-8000-00805f9b34fb");
    const BLE_MTU: usize = 247 - 3;

    let handler = get_handler().map_err(|e| format!("BLE handler unavailable: {:?}", e))?;

    // 设置notify回调
    handler
      .subscribe(READ_CHARACTERISTIC_UUID, move |data: Vec<u8>| {
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
      })
      .await
      .map_err(|e| format!("BLE subscribe failed: {:?}", e))?;

    println!("Notify callback set for transfer");
    let mut current_state = DFUState::Start;
    let mut current_block_index = 0;
    let mut last_state_change = Instant::now();
    let mut last_mcu_response_state = McuDfuState::Idle; // 初始MCU状态
    let total_blocks = (file_data.len() + DFU_PAGE_LEN - 1) / DFU_PAGE_LEN;

    let ota_result = async {
    loop {
      let mcu_response_state = *mcu_state_receiver_clone.borrow();

      if last_state_change.elapsed() > Duration::from_secs(10) {
        println!("OTA process timed out in state: {:?}", current_state);
        current_state = DFUState::Fault;
      }

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
          handler
            .send_data(
              WRITE_CHARACTERISTIC_UUID,
              "update\r\n".as_bytes(),
              WriteType::WithResponse,
            )
            .await
            .map_err(|e| format!("BLE send failed: {:?}", e))?;
          current_state = DFUState::SendPreamble;
          tokio::time::sleep(Duration::from_secs(1)).await;
          last_state_change = Instant::now();
        }
        DFUState::SendPreamble => {
          if mcu_response_state == McuDfuState::Idle {
            handler
              .send_data(
                WRITE_CHARACTERISTIC_UUID,
                &DFU_PREAMBLE,
                WriteType::WithResponse,
              )
              .await
              .map_err(|e| format!("BLE send failed: {:?}", e))?;
            println!("State: SendPreamble -> Preamble sent, waiting for MCU response");
            current_state = DFUState::SendTotalBlocks;
            last_state_change = Instant::now();
          }
        }
        DFUState::SendTotalBlocks => {
          if mcu_response_state == McuDfuState::Prepare {
            handler
              .send_data(
                WRITE_CHARACTERISTIC_UUID,
                &DFU_ACK_PATTERN.to_le_bytes(),
                WriteType::WithResponse,
              )
              .await
              .map_err(|e| format!("BLE send failed: {:?}", e))?;
            println!("State: SendTotalBlocks -> MCU Ready for Total Blocks");
            let total_blocks_bytes = (total_blocks as u32).to_le_bytes();
            handler
              .send_data(
                WRITE_CHARACTERISTIC_UUID,
                &total_blocks_bytes,
                WriteType::WithResponse,
              )
              .await
              .map_err(|e| format!("BLE send failed: {:?}", e))?;
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
            handler
              .send_data(
                WRITE_CHARACTERISTIC_UUID,
                &DFU_ACK_PATTERN.to_le_bytes(),
                WriteType::WithResponse,
              )
              .await
              .map_err(|e| format!("BLE send failed: {:?}", e))?;
            println!("State: SendBlockHeader -> MCU Ready for Block Header");
            // 模拟块头数据，实际应从固件文件中解析
            let block_header = FirmwareBlockHeader {
              signature: [0u8; 64], // 示例签名
              block_size: (file_data.len() - current_block_index * DFU_PAGE_LEN).min(DFU_PAGE_LEN)
                as u32, // 实际块大小
            };
            let block_header_bytes = block_header.to_bytes();
            handler
              .send_data(
                WRITE_CHARACTERISTIC_UUID,
                &block_header_bytes,
                WriteType::WithResponse,
              )
              .await
              .map_err(|e| format!("BLE send failed: {:?}", e))?;
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
            handler
              .send_data(
                WRITE_CHARACTERISTIC_UUID,
                &DFU_ACK_PATTERN.to_le_bytes(),
                WriteType::WithResponse,
              )
              .await
              .map_err(|e| format!("BLE send failed: {:?}", e))?;
            println!("State: SendBlockData -> MCU Ready for Block Data");
            let start = current_block_index * DFU_PAGE_LEN;
            let end = (start + DFU_PAGE_LEN).min(file_data.len());
            let block_data = &file_data[start..end];

            // 如果数据小于等于 MTU，直接发送
            if block_data.len() <= BLE_MTU {
              handler
                .send_data(
                  WRITE_CHARACTERISTIC_UUID,
                  block_data,
                  WriteType::WithResponse,
                )
                .await
                .map_err(|e| format!("BLE send failed: {:?}", e))?;
              println!("Sent chunk of size: {}", block_data.len());
            } else {
              // 数据超过 MTU，分包发送
              let chunks = block_data.chunks(BLE_MTU);
              println!(
                "Block {} Data Size: {}, Chunks: {}",
                current_block_index,
                block_data.len(),
                chunks.len()
              );
              for chunk in chunks {
                handler
                  .send_data(WRITE_CHARACTERISTIC_UUID, chunk, WriteType::WithResponse)
                  .await
                  .map_err(|e| format!("BLE chunk send failed: {:?}", e))?;
                println!("Sent chunk of size: {}", chunk.len());
              }
            }

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
            handler
              .send_data(
                WRITE_CHARACTERISTIC_UUID,
                &DFU_ACK_PATTERN.to_le_bytes(),
                WriteType::WithResponse,
              )
              .await
              .map_err(|e| format!("BLE send failed: {:?}", e))?;
            println!(
              "State: WAIT_VERIFY -> MCU Verify Block {}",
              current_block_index
            );
            current_state = DFUState::WaitWrite;
            last_state_change = Instant::now();
          }
        }
        DFUState::WaitWrite => {
          if mcu_response_state == McuDfuState::Write {
            handler
              .send_data(
                WRITE_CHARACTERISTIC_UUID,
                &DFU_ACK_PATTERN.to_le_bytes(),
                WriteType::WithResponse,
              )
              .await
              .map_err(|e| format!("BLE send failed: {:?}", e))?;
            println!(
              "State: WAIT_WRITE -> MCU Write Block {}",
              current_block_index
            );
            current_block_index += 1;
            if current_block_index >= total_blocks {
              current_state = DFUState::Complete;
            } else {
              let progress_percentage =
                ((current_block_index as f64 / total_blocks as f64) * 100.0) as u32;
              app_handle
                .emit("ota_progress", progress_percentage)
                .map_err(|e| format!("Failed to emit OTA progress: {}", e))?;
              current_state = DFUState::SendBlockHeader; // 准备发送下一个块的头部
            }
            last_state_change = Instant::now();
          }
        }
        DFUState::Complete => {
          println!("OTA process completed successfully for file: {}", file_path);
          app_handle
            .emit("ota_progress", 100)
            .map_err(|e| format!("Failed to emit OTA progress: {}", e))?;
          return Ok(());
        }
        DFUState::Fault => {
          println!("OTA process failed for file: {}", file_path);
          app_handle
            .emit("ota_progress", 0)
            .map_err(|e| format!("Failed to emit OTA progress: {}", e))?;
          app_handle
            .emit("ota_error", "OTA process failed")
            .map_err(|e| format!("Failed to emit OTA error: {}", e))?;
          return Err("OTA process failed".to_string());
        }
      }
      tokio::time::sleep(Duration::from_millis(5)).await; // 避免忙循环，等待MCU响应
    }
    }.await;

    // 无论OTA过程成功或失败，都尝试停止通知
    if let Err(e) = handler.unsubscribe(READ_CHARACTERISTIC_UUID).await {
        eprintln!("Failed to unsubscribe BLE: {}", e);
    }
    ota_result
  }
}
