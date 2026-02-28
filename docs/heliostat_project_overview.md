# Heliostat Project Summary for Claude Code

> **Agent note:** The canonical hardware inventory is [`electric-equipment.md`](electric-equipment.md).
> Consult it before making any hardware assumptions — part numbers, pin assignments,
> and specs there override anything in this summary. Treat it the way you would treat
> an API spec for a digital project.

## Project Overview

Building a dual-axis heliostat — a mirror that tracks the sun and reflects sunlight to a fixed target point. Uses two stepper motors (azimuth + elevation) controlled by an Arduino Nano ESP32 (ESP32-S3).

**Primary use case:** Reflecting sunlight indoors for natural lighting


---

## Mechanical Design

### Configuration: Alt-Azimuth Mount

```
                    ┌─────────────────┐
                    │     MIRROR      │
                    │   300×300mm     │
                    └────────┬────────┘
                             │
                    ┌────────┴────────┐
                    │  ELEVATION AXIS │ ←── Motor 2 + Worm Gear
                    │   (tilt 0-90°)  │
                    └────────┬────────┘
                             │
                    ┌────────┴────────┐
                    │   AZIMUTH AXIS  │ ←── Motor 1 + Worm Gear  
                    │  (rotate ±90°)  │
                    └────────┬────────┘
                             │
                    ┌────────┴────────┐
                    │   BASE PLATE    │
                    └─────────────────┘
```

### Key Mechanical Specs

- **Mirror:** 300×300mm (12"×12") acrylic or glass
- **Azimuth range:** ±90° (180° total)
- **Elevation range:** 0° to 90°
- **Gear ratio:** 49:1 worm gearbox (self-locking) — model 42HB48-49R
- **Resolution:** ~0.037° per step (9,800 steps/revolution)
- **Counterweight:** Required to balance mirror on elevation axis

### Why Worm Gears

1. **Self-locking** — holds position without power (critical for wind resistance)
2. **High torque** — small motor can hold heavy mirror
3. **Compact** — right-angle output saves space

---

## Hardware Components

### Electronics Inventory (see [`electric-equipment.md`](electric-equipment.md) for full specs)

| Part # | Component | Status |
|--------|-----------|--------|
| E-01 | Stepper Motor — Azimuth (42HB48-49R, 49:1 worm gear) | In Inventory |
| E-02 | Stepper Motor — Elevation (42HB48-49R, 49:1 worm gear) | In Inventory |
| E-03 | Arduino Nano ESP32 (ESP32-S3) | In Inventory |
| E-04 | TMC2209 V1.3 Driver — Azimuth | In Inventory |
| E-05 | TMC2209 V1.3 Driver — Elevation | In Inventory |
| E-06 | DS3231 RTC (primary) | In Inventory |
| E-07 | DS3231 RTC (spare) | In Inventory |
| E-08 | 12V 5A PSU (ALT-1205) | In Inventory |
| E-09 | Breadboard + jumper wires kit | In Inventory |
| E-10 | USB-C programming cable | In Inventory |
| E-11 | 100µF 25V decoupling capacitors | On Order |
| | **Total Electronics: ~$155** | |

### Mechanical Bill of Materials - proposed, to be updated with final parts

| Component | Qty | Purpose | Est. Cost |
|-----------|-----|---------|-----------|
| 2020 Aluminum Extrusion 300mm | 4 | Frame structure | $15 |
| 2020 Corner Brackets | 20 | Frame joints | $10 |
| M5 T-Nuts + Screws | 50 | Fastening | $16 |
| 608ZZ Bearings (8mm ID) | 4-6 | Shaft support | $8 |
| Shaft Collars 8mm | 4 | Shaft retention | $7 |
| Aluminum Base Plate 200×200mm | 1 | Base | $15 |
| Acrylic Mirror 12"×12" | 1 | Reflector | $15 |
| Plywood backing 12"×12" | 1 | Mirror support | $5 |
| Counterweight | 1 | Balance | $5-10 |

---

## Wiring Diagram

```
                           E-08: 12V POWER SUPPLY
                                     │
                    ┌────────────────┼────────────────┐
                    │                │                │
                    ▼                ▼                ▼
              ┌──────────┐    ┌───────────┐    ┌───────────┐
              │   E-03   │    │   E-04    │    │   E-05    │
              │  Arduino │    │ TMC2209   │    │ TMC2209   │
              │ Nano ESP32    │ (Azimuth) │    │(Elevation)│
              │          │    │           │    │           │
         ┌────┤ VIN ◄────┼────┤ VMOT ◄────┼────┤ VMOT ◄────┼── 12V
         │    │ GND ─────┼────┤ GND ──────┼────┤ GND ──────┼── 0V
         │    │          │    │           │    │           │
         │    │ D2  ─────┼───▶│ STEP      │    │           │
         │    │ D3  ─────┼───▶│ DIR       │    │           │
         │    │          │    │           │    │           │
         │    │ D4  ─────┼────┼───────────┼───▶│ STEP      │
         │    │ D5  ─────┼────┼───────────┼───▶│ DIR       │
         │    │          │    │           │    │           │
         │    │ A4  ◄────┼────┼─── SDA ───┼────┘           │
         │    │ A5  ◄────┼────┼─── SCL ───┘                │
         │    └──────────┘    │           │    │           │
         │          ▲         └─────┬─────┘    └─────┬─────┘
         │          │               │                │
         │    ┌─────┴─────┐         ▼                ▼
         │    │   E-06    │    ┌─────────┐      ┌─────────┐
         │    │  DS3231   │    │  E-01   │      │  E-02   │
         │    │   RTC     │    │ MOTOR 1 │      │ MOTOR 2 │
         │    │ VCC◄──3.3V│    │(4 wires)│      │(4 wires)│
         │    │ GND◄──GND │    └─────────┘      └─────────┘
         │    └───────────┘
         │
         └─── All GNDs connect together (common ground)
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

## Software Architecture


### Rust on ESP32

```
esp-rs ecosystem
├── esp-idf-hal (GPIO, timers)
├── esp-idf-svc (WiFi, NTP)
├── embedded-hal (hardware abstraction)
└── Custom code
    ├── Solar position algorithm (Grena3 via solar-positioning crate)
    ├── Stepper control (write from scratch)
    └── RTC interface (ds323x crate exists)
```

ESP32 has good Rust support via esp-rs project. User would need to:
- Using Grena3 algorithm via `solar-positioning` crate v0.3**
- Write stepper control logic (no AccelStepper equivalent)
- Use existing ds323x crate for RTC?

---

## Core Algorithm: Heliostat Mirror Positioning

### The Math

A heliostat must position the mirror so it reflects sunlight to a fixed target. The mirror normal must be the **bisector** of:
- Vector from mirror to sun
- Vector from mirror to target

```
        SUN ☀️
         \
          \  incident ray
           \
            \
        ─────●───── MIRROR (normal = n)
            /
           /  reflected ray
          /
         ▼
      TARGET 🎯
```

### Algorithm Steps

```
1. INPUT:
   - Current time (from RTC)
   - Location (latitude, longitude) — via env vars - possible future automation
   - Target position (azimuth, elevation relative to mirror)

2. CALCULATE SUN POSITION:
   - Usinig grena3
   - Output: sun_azimuth, sun_elevation (in degrees)

3. CONVERT TO VECTORS:
   - sun_vector = spherical_to_cartesian(sun_azimuth, sun_elevation)
   - target_vector = spherical_to_cartesian(target_azimuth, target_elevation)

4. CALCULATE MIRROR NORMAL:
   - mirror_normal = normalize(sun_vector + target_vector)

5. CONVERT BACK TO ANGLES:
   - mirror_azimuth, mirror_elevation = cartesian_to_spherical(mirror_normal)

6. MOVE MOTORS:
   - Calculate steps needed from current position
   - Send step pulses to drivers
```

## Files in This Directory

- `electric-equipment.md` — Canonical electrical parts inventory (E-01 through E-11) with full specs, wiring, and budget

---

## Reference Links

- ESP32 Rust: https://github.com/esp-rs
- Grena3 Solar Position Algorithm (selected): https://docs.rs/solar-positioning/
- NREL SPA Algorithm (considered, not used): https://midcdmz.nrel.gov/spa/
- TMC2209 Datasheet: https://www.trinamic.com/products/integrated-circuits/details/tmc2209-la/
- StepperOnline (motors): https://www.omc-stepperonline.com/
