# BLE OTA / DFU

The Tauri OTA state machine starts by sending `update\r\n` over the active BLE
transfer. It then sends the four-byte DFU preamble `AA 55 AA 55` and waits for
the MCU to notify `PREPARE` (state `1`).

The bootloader may need time to power up the BLE module after the update reset.
While the MCU remains in `Idle`, the Tauri side resends the preamble every 500 ms.
Once `PREPARE` arrives, the normal sequence continues:

`total blocks -> block header -> block data -> verify -> write -> next block`

The MCU state bytes are defined by `applications/bootdfu.c`. In particular,
`Final` is `6` and `Error` is `8`. Unknown notification bytes are rejected and
logged so a malformed notification cannot silently restart the transfer as if
the MCU were idle.

The OTA loop still fails after 60 seconds without a state transition. BLE
transport errors retain the existing three-attempt reconnect behavior.
