# First-Flash Guide: Arduino Nano ESP32 with ESP-IDF

How to replace the Arduino bootloader with ESP-IDF firmware on a fresh Arduino Nano ESP32 board. This is a **one-time process** — subsequent flashes use `cargo run`.

## Prerequisites

**Tools** (on your Linux build machine):

```bash
# arduino-cli
curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | sh
export PATH=$PATH:$HOME/bin

# ESP32 Arduino core
arduino-cli config init
arduino-cli config add board_manager.additional_urls \
  https://raw.githubusercontent.com/espressif/arduino-esp32/gh-pages/package_esp32_index.json
arduino-cli core update-index
arduino-cli core install esp32:esp32

# dfu-util
sudo apt-get install -y dfu-util

# esptool
pip3 install esptool --break-system-packages --ignore-installed cryptography
```

**ESP-IDF Rust toolchain** already set up (`espup`, `espflash`, `ldproxy`, etc).

**WSL2 users**: USB passthrough must be configured — see [setup-arduino.md](setup-arduino.md).

**Firmware config** (already done in this repo):

- `sdkconfig.defaults` includes `CONFIG_ESP_CONSOLE_USB_SERIAL_JTAG=y`
- `.cargo/config.toml` runner uses `--before usb-reset`

## The Problem

The Arduino Nano ESP32 ships with an **Arduino DFU bootloader**. When you double-click RST, it enters Arduino DFU mode (`2341:0070`), which only accepts firmware via `dfu-util`.

But `espflash` and `esptool` need **ESP ROM download mode** (`303a:0009`). The usual B1+RST pin method to enter ROM download mode is unreliable on this board.

## The Solution: Software-Triggered ROM Download Mode

### Step 1 — Build the ESP-IDF firmware

Clean and rebuild (needed if you changed `sdkconfig.defaults`):

```bash
source ~/export-esp.sh && source ~/.cargo/env
cd ~/rusty-heliostat/firmware
cargo clean -p esp-idf-sys
WIFI_SSID=<your_ssid> WIFI_PASSWORD=<your_password> cargo build
```

### Step 2 — Create a merged firmware binary

```bash
espflash save-image --chip esp32s3 --merge --skip-padding \
  --flash-size 8mb \
  target/xtensa-esp32s3-espidf/debug/firmware \
  merged_firmware.bin
```

`--skip-padding` is critical — without it the image is ~8 MB and too large for DFU.

### Step 3 — Create the ROM download mode sketch

Create `~/blink/blink.ino`:

```cpp
#include "soc/rtc_cntl_reg.h"
#include "esp_system.h"

void setup() {
  Serial.begin(115200);
  delay(3000);
  Serial.println("Entering ROM download mode...");
  delay(100);
  // Force chip into ROM download mode on next reset
  REG_WRITE(RTC_CNTL_OPTION1_REG, RTC_CNTL_FORCE_DOWNLOAD_BOOT);
  esp_restart();
}

void loop() {}
```

Compile it:

```bash
arduino-cli compile --fqbn esp32:esp32:nano_nora ~/blink
```

Note the `.bin` path in the output — it will be something like:
`~/.cache/arduino/sketches/<HASH>/blink.ino.bin`

### Step 4 — Enter Arduino DFU mode

**Double-click RST** on the board. The LED may pulse or change color.

Verify:

```bash
lsusb | grep 2341:0070
# Expected: Arduino SA ARDUINO_NANO_NORA
```

### Step 5 — Flash the download-mode sketch via DFU

```bash
sudo dfu-util --device 0x2341:0x0070 -a 0 \
  -D ~/.cache/arduino/sketches/<HASH>/blink.ino.bin -R
```

Replace `<HASH>` with the actual path from Step 3.

The `-R` flag resets the board after flashing. The sketch boots, waits 3 seconds, then triggers ROM download mode.

### Step 6 — Wait for ROM download mode

Wait ~5 seconds. If the board disappears from USB entirely, **unplug and replug the cable**.

Verify:

```bash
lsusb | grep 303a:0009
# Expected: Espressif ESP32-S3
```

### Step 7 — Flash the ESP-IDF firmware with esptool

```bash
sudo esptool --chip esp32s3 --port /dev/ttyACM0 \
  --before no-reset --no-stub \
  write-flash --flash-size 8MB 0x0 merged_firmware.bin
```

Key flags:
- `--before no-reset` — board is already in download mode
- `--no-stub` — required; the flash stub fails over USB on this board

This takes ~35 seconds.

### Step 8 — Boot the new firmware

**Press RST once.** The ESP-IDF firmware boots.

Verify:

```bash
lsusb | grep 303a:1001
# Expected: Espressif USB JTAG/serial debug unit
```

Monitor serial output:

```bash
espflash monitor --port /dev/ttyACM0
```

## Subsequent Flashes

No manual steps needed. Just run:

```bash
source ~/export-esp.sh && source ~/.cargo/env
cd ~/rusty-heliostat/firmware
WIFI_SSID=<your_ssid> WIFI_PASSWORD=<your_password> cargo run
```

The `--before usb-reset` runner flag auto-enters download mode via the USB Serial/JTAG controller.

## USB Device ID Reference

| VID:PID     | Mode                    | Meaning                                |
|-------------|-------------------------|----------------------------------------|
| `2341:0070` | Arduino DFU bootloader  | Double-click RST on stock board        |
| `303a:0009` | ESP ROM download mode   | What esptool/espflash need to flash    |
| `303a:1001` | USB JTAG/serial debug   | ESP-IDF firmware running normally      |

## Gotchas

- **Don't use `espflash` for the first flash** — it connects but the flash stub fails over USB. Use `esptool --no-stub` instead.
- **Don't use `--dfuse-address`** with dfu-util — the Arduino DFU bootloader is not a DfuSe device.
- **`dfu-util` needs sudo** — without it you get `LIBUSB_ERROR_ACCESS`.
- **Board may vanish from USB** after the download-mode sketch runs — just unplug and replug.
- **Use `esptool` not `esptool.py`** — the `.py` suffix is deprecated.

## Recovery

If you need to restore the Arduino bootloader:
[Reset the Arduino bootloader on the Nano ESP32](https://support.arduino.cc/hc/en-us/articles/9810414060188-Reset-the-Arduino-bootloader-on-the-Nano-ESP32)
