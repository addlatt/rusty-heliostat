# Rusty Heliostat

A dual-axis heliostat controller written in Rust, running on an Arduino Nano ESP32 (ESP32-S3) via ESP-IDF.

The heliostat tracks the sun and angles a mirror to reflect sunlight at a fixed target point — useful for redirecting natural light indoors.

## Hardware

- **MCU**: Arduino Nano ESP32 (ESP32-S3, 8 MB flash, 8 MB PSRAM)
- **Motor drivers**: 2x TMC2209 V1.3 (STEP/DIR)
- **Motors**: 2x NEMA 17 with 49:1 worm gearboxes (azimuth + elevation)
- **RTC**: DS3231 (I2C)
- **Power**: 12V 5A supply

See [docs/electric-equipment.md](docs/electric-equipment.md) for full specs and [docs/heliostat_project_summary_1.md](docs/heliostat_project_summary_1.md) for the overall project design.

## Firmware Features

- WiFi connectivity (WPA2)
- HTTP status endpoint (`GET /`)
- OTA firmware updates (`POST /flash`)
- OTA rollback protection
- Heartbeat LED on GPIO 48

## Getting Started

### Prerequisites

- [espup](https://github.com/esp-rs/espup) toolchain (`espup install`)
- `espflash`, `ldproxy` (`cargo install espflash ldproxy`)
- WiFi credentials set as env vars: `WIFI_SSID`, `WIFI_PASSWORD`

### First Flash (fresh Arduino Nano ESP32)

The board ships with an Arduino DFU bootloader that isn't compatible with `espflash`. A one-time software workaround is needed to switch to ESP-IDF.

Follow [docs/first-flash-guide.md](docs/first-flash-guide.md) for the complete walkthrough.

### Build & Flash (after first flash)

```bash
source ~/export-esp.sh && source ~/.cargo/env
cd firmware
WIFI_SSID=<ssid> WIFI_PASSWORD=<password> cargo run
```

`cargo run` flashes and opens the serial monitor automatically.

### OTA Deploy

After the device is on your network:

```bash
cd firmware
WIFI_SSID=<ssid> WIFI_PASSWORD=<password> ./deploy.sh <device-ip>
```

## Project Structure

```
rusty-heliostat/
├── firmware/
│   ├── src/main.rs           # Application entry point
│   ├── Cargo.toml            # Dependencies (esp-idf-svc, anyhow, log)
│   ├── sdkconfig.defaults    # ESP-IDF config (8 MB flash, USB JTAG console, OTA)
│   ├── partitions.csv        # Dual OTA partition layout
│   ├── deploy.sh             # OTA deploy script
│   ├── .cargo/config.toml    # Build target, runner, espflash flags
│   └── rust-toolchain.toml   # Xtensa toolchain pinning
└── docs/
    ├── first-flash-guide.md          # One-time Arduino-to-ESP-IDF flashing guide
    ├── setup-arduino.md              # WSL2 USB forwarding & arduino-cli setup
    ├── electric-equipment.md         # Hardware inventory & wiring
    └── heliostat_project_summary_1.md # Project design & architecture
```
