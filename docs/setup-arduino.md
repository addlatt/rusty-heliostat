# Arduino Nano ESP32 Setup Plan

## Goal
Establish a working dev connection from WSL2 to the Arduino Nano ESP32 (E-03) over USB.

## Current State
- E-03 (Arduino Nano ESP32) powers on via USB-C — flashing lights confirmed
- E-10 (USB-C cable) connected to laptop
- WSL2 cannot see USB devices natively — `esptool` finds 0 serial ports
- Python venv created at `~/venv` with `esptool` installed

## Step 1: Install usbipd-win (Windows side)

Open **PowerShell as Administrator**:

```powershell
winget install usbipd
```

Reboot if prompted.

## Step 2: Forward Arduino USB to WSL2

In **PowerShell as Administrator**:

```powershell
# List USB devices — find the Arduino (look for "USB Serial" or "ESP32")
usbipd list

# Bind the device (one-time, replace <BUSID> with actual bus ID, e.g. 1-3)
usbipd bind --busid <BUSID>

# Attach to WSL (run this each time you reconnect the cable)
usbipd attach --wsl --busid <BUSID>
```

## Step 3: Verify serial port in WSL

Back in **WSL2**:

```bash
ls /dev/ttyACM* /dev/ttyUSB*
```

One of these should now exist (likely `/dev/ttyACM0`).

If permission denied:
```bash
sudo usermod -aG dialout $USER
# Then log out and back into WSL
```

## Step 4: Verify chip communication

```bash
source ~/venv/bin/activate
esptool chip-id
```

Expected output: ESP32-S3 chip type, MAC address, and flash size.

## Step 5: Upload a blink test

Minimal verification that code upload works. Two options:

### Option A: Arduino CLI (no GUI needed)

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/arduino/arduino-cli/master/install.sh | sh
export PATH="$PATH:$HOME/bin"

# Set up ESP32-S3 board support
arduino-cli config init
arduino-cli config add board_manager.additional_urls \
  https://raw.githubusercontent.com/espressif/arduino-esp32/gh-pages/package_esp32_index.json
arduino-cli core update-index
arduino-cli core install esp32:esp32

# Create and upload blink sketch
mkdir -p ~/heliostat/firmware/blink && cat > ~/heliostat/firmware/blink/blink.ino << 'EOF'
void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(115200);
  Serial.println("Blink started");
}
void loop() {
  digitalWrite(LED_BUILTIN, HIGH);
  delay(500);
  digitalWrite(LED_BUILTIN, LOW);
  delay(500);
}
EOF

# Upload (adjust port if needed)
arduino-cli compile --fqbn esp32:esp32:arduino_nano_esp32 ~/heliostat/firmware/blink
arduino-cli upload --fqbn esp32:esp32:arduino_nano_esp32 --port /dev/ttyACM0 ~/heliostat/firmware/blink

# Watch serial output
arduino-cli monitor --port /dev/ttyACM0 --config baudrate=115200
```

### Option B: Rust (esp-rs) — skip to this if you want to go straight to Rust

```bash
# Install Rust + ESP toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install espup
espup install
source ~/export-esp.sh

# Install cargo-espflash for uploading
cargo install cargo-espflash

# Create a project (use esp-idf template for ESP32-S3)
cargo install cargo-generate
cargo generate esp-rs/esp-idf-template cargo --name blink
cd blink

# Edit src/main.rs to blink, then:
cargo build
cargo espflash flash --monitor --port /dev/ttyACM0
```

## Done When
- [ ] `esptool chip-id` returns ESP32-S3 info from WSL
- [ ] Blink sketch uploaded and LED pattern visibly changes
- [ ] Serial output received ("Blink started")
