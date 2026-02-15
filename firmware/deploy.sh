#!/usr/bin/env bash
set -euo pipefail

DEVICE_IP="${1:?Usage: ./deploy.sh <device-ip>}"

echo "==> Building release firmware..."
cargo build --release

echo "==> Saving OTA image..."
espflash save-image --chip esp32s3 target/xtensa-esp32s3-espidf/release/firmware target/firmware_ota.bin

echo "==> Flashing via OTA to ${DEVICE_IP}..."
curl -X POST --data-binary @target/firmware_ota.bin "http://${DEVICE_IP}/flash" -w "\nHTTP %{http_code}\n"

echo "==> Done! Device should reboot with new firmware."
