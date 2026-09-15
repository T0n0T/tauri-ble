# Changelog

## v0.3.3 - 2026-09-15

### BLE OTA / DFU

- Removed the fixed one-second pause after sending the `update` command.
- Retry the `AA 55 AA 55` DFU preamble every 500 ms until the MCU reports `PREPARE`.
- Interpret MCU DFU state `8` as `Fault`, matching the bootloader enum.
- Reject unknown MCU state bytes instead of treating them as `Idle`.

This handles the bootloader startup window in which the BLE module is powering up
and the first preamble can arrive before the MCU's DFU receiver is ready.
