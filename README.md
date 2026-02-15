# Rusty Heliostat
An under construction dual-axis heliostat controller written in Rust, running on an Arduino Nano ESP32 (ESP32-S3) via ESP-IDF. Aiming to open source the project, including software and hardware levels. 

The heliostat will track the sun and angle a mirror to reflect sunlight at a fixed target point — useful for redirecting natural light indoors. Hoping I can set this up to rotate to a few fixed spots throughout the course of the day. I have some plants inside that sorely need the light. 

## Hardware

- **MCU**: Arduino Nano ESP32 (ESP32-S3, 8 MB flash, 8 MB PSRAM)
- **Motor drivers**: 2x TMC2209 V1.3 (STEP/DIR)
- **Motors**: 2x NEMA 17 with 49:1 worm gearboxes (azimuth + elevation)
- **RTC**: DS3231 (I2C)
- **Power**: 12V 5A supply

See [docs/electric-equipment.md](docs/electric-equipment.md) for full specs.

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

