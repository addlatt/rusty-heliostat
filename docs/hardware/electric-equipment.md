# Heliostat Electrical Equipment

## Parts List

### E-01: Stepper Motor — Azimuth — ✅ In Inventory
- **Model:** 42HB48-49R (NEMA 17 Worm Gear)
- **Role:** Azimuth axis
- **Type:** Hybrid Bipolar, 4-lead
- **Step angle:** 1.8°
- **Current:** 1.5A/phase
- **Voltage:** 12-36V DC
- **Motor torque:** 5.5kg.cm
- **Reduction ratio:** 49:1
- **Output shaft:** 8mm diameter, 15mm length
- **Motor size:** 42mm × 42mm × 48mm
- **Wire colors:** Red, Green, Blue, Black

### E-02: Stepper Motor — Elevation — ✅ In Inventory
- **Model:** 42HB48-49R (NEMA 17 Worm Gear)
- **Role:** Elevation axis
- _(same specs as E-01)_
- **Wire colors:** Red, Green, Blue, Black

### E-03: Microcontroller — ✅ In Inventory
- **Arduino Nano ESP32** — $20.60 (incl. shipping/tax)
  - **Chip:** ESP32-S3
  - **Connectivity:** WiFi, Bluetooth LE
  - **RAM:** 512KB
  - **Flash:** 8MB internal, 16MB external
  - **Voltages:** 5V (USB), 3.3V (operational), 6-21V (VIN input)
  - **Pinout:** 14 digital, 8 analog
  - Header pins: 2×15, loose _(soldering required before breadboard use)_
  - Source: Arduino Store

### E-04: Motor Driver — Azimuth — ✅ In Inventory
- **BIGTREETECH TMC2209 V1.3** — ~$15 (pair)
  - Role: Drives E-01
  - 2A continuous, 2.8A peak
  - Ultra quiet, 256 microsteps
  - STEP/DIR and UART mode
  - Source: Amazon

### E-05: Motor Driver — Elevation — ✅ In Inventory
- **BIGTREETECH TMC2209 V1.3**
  - Role: Drives E-02
  - _(same specs as E-04)_

### E-06: RTC Module (primary) — ✅ In Inventory
- **DORHEA DS3231 AT24C32** — ~$7 (pair)
  - Temperature-compensated, battery backup
  - I2C interface (A4/A5)
  - 3.3-5.5V compatible
  - Includes cable
  - Source: Amazon

### E-07: RTC Module (spare) — ✅ In Inventory
- **DORHEA DS3231 AT24C32**
  - _(same specs as E-06, backup unit)_
  - Includes cable

### E-08: Power Supply — ✅ In Inventory
- **AC/DC Adapter ALT-1205** — ~$12
  - Input: AC 100-240V, 50/60Hz
  - Output: DC 12V, 5A (60W)
  - Bare wires + quick connector included
  - **Polarity: Red = +, Black = −**
  - Source: Amazon

### E-09: Breadboard + Jumper Wires Kit — ✅ In Inventory
- **QCCAN Breadboard Jumper Wires Kit** — ~$16
  - 2× 830-point + 2× 400-point breadboards
  - 560× U-shape jumper wires (14 lengths)
  - 65× M/M flexible jumper wires
  - 3× 20-pin dupont wires (F/F, F/M, M/M)
  - Tweezers included
  - Source: Amazon

### E-10: Programming Cable — ✅ In Inventory
- **Arduino Official USB-C 2-in-1 Cable** — ~$8
  - USB-C to USB-C + USB-A adapter
  - 100cm length
  - Data + power
  - Source: Amazon

### E-11: VMOT Decoupling Capacitors — ⬜ On Order
- **100µF 25V Electrolytic Capacitors** (20 pack) — ~$6
  - Size: 6×11mm
  - Need 2 (one per TMC2209 driver, on VMOT/GND)
  - 18 spares
  - Source: Amazon

---

## Wiring Overview

```
E-08 (PSU) ──┬── VIN → E-03 (Arduino Nano ESP32)
             ├── VMOT → E-04 (TMC2209 — Azimuth)
             └── VMOT → E-05 (TMC2209 — Elevation)

GND ────── All grounds tied together
```

### Pin Assignments (E-03: Arduino Nano ESP32)

| Pin | Function | Connects To |
|-----|----------|-------------|
| VIN | 12V Power | E-08 PSU 12V+ |
| GND | Ground | Common ground |
| D2 | Azimuth STEP | E-04 STEP |
| D3 | Azimuth DIR | E-04 DIR |
| D4 | Elevation STEP | E-05 STEP |
| D5 | Elevation DIR | E-05 DIR |
| A4 | I2C SDA | E-06 SDA |
| A5 | I2C SCL | E-06 SCL |

---

## Budget

| Part | Item | Est. Cost | Status |
|------|------|-----------|--------|
| E-01 | Stepper Motor — Azimuth (42HB48-49R) | ~$35 | In Inventory |
| E-02 | Stepper Motor — Elevation (42HB48-49R) | ~$35 | In Inventory |
| E-03 | Arduino Nano ESP32 | $20.60 | In Inventory |
| E-04 | TMC2209 Driver — Azimuth | ~$7.50 | In Inventory |
| E-05 | TMC2209 Driver — Elevation | ~$7.50 | In Inventory |
| E-06 | DS3231 RTC (primary) | ~$3.50 | In Inventory |
| E-07 | DS3231 RTC (spare) | ~$3.50 | In Inventory |
| E-08 | 12V 5A PSU (ALT-1205) | ~$12 | In Inventory |
| E-09 | Breadboard + wires kit | ~$16 | In Inventory |
| E-10 | USB-C cable | ~$8 | In Inventory |
| E-11 | 100µF 25V Capacitors (×20) | ~$6 | On Order |
| | **Total** | **~$155** | |
