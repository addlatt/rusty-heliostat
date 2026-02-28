# Breadboard Wiring Guide: Rusty Heliostat

> **For absolute beginners.** No prior electronics experience needed.
> Read each section fully before doing anything with your hands.

---

## Table of Contents

1. [Before You Start — Safety](#1-before-you-start--safety)
2. [Know Your Parts](#2-know-your-parts)
3. [How a Breadboard Works](#3-how-a-breadboard-works)
4. [How Jumper Wires Work](#4-how-jumper-wires-work)
5. [Prerequisite: Solder Headers on the Arduino](#5-prerequisite-solder-headers-on-the-arduino)
6. [Plan Your Layout](#6-plan-your-layout)
7. [Step-by-Step Wiring](#7-step-by-step-wiring)
   - Phase 1: Place the Components
   - Phase 2: Power Rails
   - Phase 3: Power the Motor Drivers
   - Phase 4: Signal Wires (STEP/DIR)
   - Phase 5: Wire the RTC Module
   - Phase 6: Motor Coil Wires
   - Phase 7: Driver Enable Jumpers
   - Phase 8: Decoupling Capacitors
8. [Final Checklist Before Powering On](#8-final-checklist-before-powering-on)
9. [First Power-On](#9-first-power-on)
10. [Troubleshooting](#10-troubleshooting)
11. [Complete Wiring Reference Table](#11-complete-wiring-reference-table)

---

## 1. Before You Start — Safety

**Read this. It matters.**

- **Never wire anything while power is connected.** Always unplug the 12V power supply
  AND the USB cable before adding or moving wires. This is the single most important rule.
- **12V will not hurt you.** The voltages in this project (3.3V, 5V, 12V) are safe to touch.
  You won't get shocked. But a wrong connection can destroy your components instantly.
- **Static electricity can kill chips.** Before handling the Arduino or TMC2209 drivers,
  touch a metal object (doorknob, metal table leg) to discharge static from your body.
- **Double-check every wire before powering on.** A single wrong connection can fry a $20
  microcontroller. Go slow.
- **The motors and power supply are the only things that can hurt you physically.** The
  motors can pinch your fingers if they spin unexpectedly. The power supply converts wall
  power (120/240V AC) — never open its case or touch its internals.

---

## 2. Know Your Parts

Lay out every part on your desk and identify each one before touching the breadboard.

### E-03: Arduino Nano ESP32 (Your Brain)

```
What it looks like:
┌──────────────────────────────────────────┐
│  [USB-C Port]                            │
│                                          │
│  ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●  ← 15 pin holes (top edge)
│                                          │
│         (chip and components)            │
│                                          │
│  ● ● ● ● ● ● ● ● ● ● ● ● ● ● ●  ← 15 pin holes (bottom edge)
│                                          │
└──────────────────────────────────────────┘
          ~45mm long × ~18mm wide
```

- **Tiny blue/green circuit board** about the size of a stick of gum
- **USB-C port** on one end (for programming and power during development)
- **Two rows of 15 holes** along the long edges — this is where you'll solder the header pins
- Each hole has a **label printed on the board** (D2, D3, A4, A5, VIN, GND, etc.)
- Look at both sides of the board to find the labels — they may be on the bottom

### E-04 / E-05: TMC2209 Motor Drivers (x2)

```
What it looks like:
┌────────────────────────┐
│   [heatsink on top]    │
│                        │
│  ●  label  label  ●   │
│  ●  label  label  ●   │  ← 8 pins on each side (16 total)
│  ●  label  label  ●   │
│  ●  label  label  ●   │
│  ●  label  label  ●   │
│  ●  label  label  ●   │
│  ●  label  label  ●   │
│  ●  label  label  ●   │
│                        │
│   [trimpot on bottom]  │  ← small adjustable screw
└────────────────────────┘
       ~20mm × ~15mm
```

- **Small board** with a **metal heatsink** on top (may be pre-attached or need to be stuck on)
- **Two rows of 8 pins** (16 total) with labels printed next to each pin
- A **tiny screw (trimpot/potentiometer)** on one end for adjusting motor current
- **You have two of these** — one for azimuth (E-04), one for elevation (E-05)
- They look identical. Label them with tape: write "AZ" on one and "EL" on the other

**Find these pin labels on your TMC2209 boards** (they are printed on the PCB silkscreen).

The BIGTREETECH TMC2209 V1.3 has a **red side** and a **black side**:

```
Red side (signal/logic):   Black side (power/motor):
  EN                          VS    ← motor voltage (12V)
  MS1                         GND   ← motor ground
  MS2                         A2    ← motor coil A
  RX                          A1    ← motor coil A
  TX                          B1    ← motor coil B
  CLK                         B2    ← motor coil B
  STEP                        VIO   ← logic voltage (3.3V)
  DIR                         GND   ← logic ground
```

> **Label translation:** Your board says **VS** (other guides call it VM or VMOT).
> Your board says **VIO** (other guides call it VDD). **A1/A2** = coil A pair,
> **B1/B2** = coil B pair (other guides use 1A/2A/1B/2B).

> **Important:** The exact order may vary slightly between batches. **Always read the
> labels printed on YOUR specific board.** The labels are small — use a magnifying glass
> or phone camera zoom if needed.

### E-06: DS3231 RTC Module (Your Clock)

```
What it looks like:
┌──────────────────────────────┐
│                              │
│   [battery holder]  [chip]   │
│                              │
│   ● ● ● ● ● ●              │  ← 6 pins in a row (or 4, varies by model)
│  32K SQW SCL SDA VCC GND    │
└──────────────────────────────┘
         ~38mm × ~22mm
```

- **Small blue/purple board** larger than the drivers, with a **round coin battery** on one side
- **One row of pins** along one edge (typically 4-6 pins)
- Labels: **GND, VCC, SDA, SCL** (and possibly SQW, 32K which we don't use)
- The battery keeps the clock running when everything else is powered off
- May come with **pre-soldered header pins** and a cable — you can use either

### E-08: 12V Power Supply

```
What it looks like:
┌─────────────┐
│  [AC plug]  │──── wall outlet
│  (brick)    │
│             │──── two bare wires:
└─────────────┘     RED  = +12V (positive)
                    BLACK = GND  (negative/ground)
```

- **Black plastic brick** with a plug for the wall outlet
- **Two wires coming out:** Red (+12V) and Black (Ground)
- May include a quick-connect adapter — we'll just use the bare wire ends
- **Do not plug this into the wall until the very last step**

### E-09: Breadboard and Wires Kit

Your kit contains:

| Item | What it looks like | What it's for |
|------|--------------------|---------------|
| 830-point breadboard (x2) | White/clear rectangular board with lots of holes | Main building surface |
| 400-point breadboard (x2) | Smaller version of above | Not needed for this project |
| U-shape jumper wires (560) | Short stiff wires bent into a U shape, various colors | Clean short connections |
| Flexible jumper wires (65) | Longer wires with pins on each end (M/M) | Longer connections |
| Dupont wires (3x20 pin) | Ribbon cables with connectors (F/F, F/M, M/M) | Component connections |
| Tweezers | Small metal tweezers | Pulling out wires |

---

## 3. How a Breadboard Works

This is the most important concept to understand. **Take 2 minutes to really get this.**

### The Grid of Holes

A standard 830-point breadboard looks like this:

```
         Column numbers (1-63)
         1   5   10  15  20  25  30  35  40  45  50  55  60 63
         ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓   ↓  ↓
    (+) ═══════════════════════════════════════════════════════════  ← Power rail (+)
    (-) ═══════════════════════════════════════════════════════════  ← Power rail (-)
         ┌─────────────────────────────────────────────────────────
    a  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
    b  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
    c  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
    d  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
    e  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
         │
         │              ══════ CENTER GAP ══════
         │
    f  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
    g  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
    h  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
    i  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
    j  → │ ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ● ...
         └─────────────────────────────────────────────────────────
    (+) ═══════════════════════════════════════════════════════════  ← Power rail (+)
    (-) ═══════════════════════════════════════════════════════════  ← Power rail (-)
```

### The Secret: Hidden Metal Strips

Underneath every breadboard, hidden metal clips connect certain holes together.
**This is how electricity flows without soldering.**

**Rule 1 — Terminal strips (the main grid):**
Each row of 5 holes (a-b-c-d-e) in the SAME column is connected internally.

```
Column 10:
    a10 ──┐
    b10 ──┤  ALL 5 ARE
    c10 ──┤  CONNECTED
    d10 ──┤  TOGETHER
    e10 ──┘
              ← center gap breaks the connection
    f10 ──┐
    g10 ──┤  ALL 5 ARE
    h10 ──┤  CONNECTED
    i10 ──┤  TOGETHER
    j10 ──┘
```

So if you plug a wire into hole **a10** and another wire into hole **c10**, those two
wires are electrically connected — electricity can flow between them.

But **a10** and **f10** are NOT connected (the center gap separates them).
And **a10** and **a11** are NOT connected (different columns).

**Rule 2 — Power rails (the long strips on the edges):**
The (+) and (-) strips run the entire length of the board. Every hole in a (+) rail
is connected to every other hole in that same (+) rail.

```
(+) ════●════●════●════●════●════●════●════  ← ALL connected horizontally
(-) ════●════●════●════●════●════●════●════  ← ALL connected horizontally
```

> **Note:** Some breadboards have a small gap in the middle of the power rails (around
> column 30). If yours does, you'll need a short wire to bridge it. Check by looking at
> the red and blue lines printed on the breadboard — if the line has a break, so does
> the connection.

**Rule 3 — Nothing else is connected.**
Top power rails are NOT connected to bottom power rails. Left half is NOT connected to
right half (unless you add wires).

### Why This Matters

When you plug a component's pin into the breadboard, you get 4 remaining holes in that
column (same side of the gap) to connect wires to. That's how you wire things together
without soldering — you just share columns.

```
Example: Connecting an Arduino pin to a driver pin

Arduino pin goes into column 10, row c  →  c10
You plug a wire into column 10, row a   →  a10
That wire goes to the driver's column   →  Both are now connected!
```

---

## 4. How Jumper Wires Work

### U-Shape Jumper Wires (the short stiff ones)

```
    ┌──────┐
    │      │      These are pre-bent wires that lie flat on the
    │      │      breadboard. They come in different lengths.
    ●      ●      Use them for short, tidy connections.
```

- Each end has a **short metal pin** that plugs into a breadboard hole
- They come in multiple lengths and colors
- **Pick the right length** — the wire should reach between the two holes without
  being stretched tight or having lots of slack

### Flexible Jumper Wires (the long bendy ones)

```
    ●───────────────────●     Longer wires for connections that
    pin               pin     span a greater distance. Messier
                              but more flexible.
```

- **M/M (male/male):** Metal pins on both ends — plug into breadboard holes
- **F/M (female/male):** Socket on one end, pin on the other — for connecting to
  components that have pins sticking out
- **F/F (female/female):** Sockets on both ends — for connecting two pin headers

### Wire Color Convention

Using consistent colors makes debugging much easier. Follow this scheme:

| Color | Meaning | Used For |
|-------|---------|----------|
| **Red** | Power (+) | 12V supply, 3.3V/5V logic power |
| **Black** | Ground (-) | All ground connections |
| **Green** | STEP signal | Arduino → Driver STEP pins |
| **Blue** | DIR signal | Arduino → Driver DIR pins |
| **Orange** | Motor coil A | Driver 1A/1B → Motor |
| **Yellow** | Motor coil B | Driver 2A/2B → Motor |
| **Yellow** | I2C data | SDA, SCL to RTC (distinct from motor coil yellow by length/location) |
| **Gray** | Jumper/bypass | RESET-to-SLEEP jumpers on drivers |

---

## 5. Prerequisite: Solder Headers on the Arduino

Your Arduino Nano ESP32 comes with **loose header pins** — two strips of 15 pins each.
These MUST be soldered onto the board before it can plug into a breadboard.

**If you have never soldered before**, watch a 5-minute YouTube video on "how to solder
header pins to Arduino." It's one of the easiest soldering tasks.

### What You Need

- Soldering iron (any basic one, ~$15-30)
- Solder (lead-free rosin core, thin gauge like 0.8mm)
- The two 15-pin header strips that came with the Arduino
- Your breadboard (used as a jig to hold the pins straight)

### Steps

1. **Push the two header strips into the breadboard**, long pins down, spaced
   to match the two rows of holes on the Arduino (the rows are 0.6 inches / ~15mm apart).
   The breadboard holds them perfectly straight and at the right spacing.

2. **Place the Arduino on top** of the header strips. The short pins should poke up
   through the Arduino's holes. The board should sit flat on the plastic part of the headers.

3. **Solder each pin** on the TOP side of the Arduino:
   - Touch the soldering iron tip to both the pin AND the metal pad on the board
   - Wait 1-2 seconds for them to heat up
   - Touch the solder to the joint (not the iron) — it should flow and form a small
     shiny cone
   - Remove the solder, then the iron
   - The whole joint takes about 3 seconds

4. **Do all 30 pins** (15 on each side).

5. **Let it cool** for a minute, then gently pull the Arduino off the breadboard.

6. **Inspect your work.** Each joint should be a small shiny cone. If any look like a
   cold blob or have a bridge connecting two adjacent pins, reheat and fix.

> **No soldering iron?** You can buy "solderless headers" that press-fit, but they're
> less reliable. Or ask a local makerspace — they'll likely help you for free.

---

## 6. Plan Your Layout

Before plugging anything in, let's plan where each component goes on the breadboard.

### Overview — One 830-Point Breadboard

We'll use a single 830-point breadboard. Components straddle the center gap.

```
                     BREADBOARD LAYOUT (top view)
  ┌──────────────────────────────────────────────────────────────────────┐
  │ (+) ═══════════════════════════════════════════════════════ (+) rail │
  │ (-) ═══════════════════════════════════════════════════════ (-) rail │
  │                                                                      │
  │     ARDUINO             AZ DRIVER        EL DRIVER        RTC       │
  │   (col 1-15)          (col 22-29)      (col 36-43)    (col 52-57)  │
  │  a ●●●●●●●●●●●●●●●  ●●●●●●●●         ●●●●●●●●       ●●●●●●      │
  │  b ●●●●●●●●●●●●●●●  ●●●●●●●●         ●●●●●●●●       ●●●●●●      │
  │  c ●●●●●●●●●●●●●●●  ●●●●●●●●         ●●●●●●●●       ●●●●●●      │
  │  d ■■■■■■■■■■■■■■■  ●●●●●●●●         ●●●●●●●●       ●●●●●●      │
  │  e ●●●●●●●●●●●●●●●  ●●●●●●●●         ●●●●●●●●       ●●●●●●      │
  │  ────────────────────── CENTER GAP ──────────────────────────────── │
  │  f ●●●●●●●●●●●●●●●  ●●●●●●●●         ●●●●●●●●                    │
  │  g ●●●●●●●●●●●●●●●  ●●●●●●●●         ●●●●●●●●                    │
  │  h ■■■■■■■■■■■■■■■  ●●●●●●●●         ●●●●●●●●                    │
  │  i ●●●●●●●●●●●●●●●  ●●●●●●●●         ●●●●●●●●                    │
  │  j ●●●●●●●●●●●●●●●  ●●●●●●●●         ●●●●●●●●                    │
  │                                                                      │
  │ (+) ═══════════════════════════════════════════════════════ (+) rail │
  │ (-) ═══════════════════════════════════════════════════════ (-) rail │
  └──────────────────────────────────────────────────────────────────────┘

  ■ = Arduino pin (rows d and h)     Columns: 1-15 | 22-29 | 36-43 | 52-57
```

### Why This Spacing?

- **Arduino** is 15 pins wide — takes columns 1-15 (pins land in rows d and h)
- **Gap of ~5 columns** between components gives room for wires
- **Drivers** are 8 pins wide — each takes 8 columns
- **RTC** plugs into one side only (top half, rows a-e) since it has a single row of pins

---

## 7. Step-by-Step Wiring

> **IMPORTANT: Power supply must be UNPLUGGED. USB cable must be DISCONNECTED.**
>
> Confirm this now before proceeding.

---

### Phase 1: Place the Components

**This phase:** You'll push each component into the breadboard. No wires yet.

---

#### Step 1.1 — Place the Arduino Nano ESP32

**What you're doing:** Seating the Arduino so its pins go into the breadboard.

1. Hold the Arduino with the **USB-C port pointing LEFT** (toward column 1).
2. The two rows of header pins should straddle the **center gap**.
3. Align the first pin on each side with **column 1**.
4. **Gently but firmly press down** until the Arduino sits flat. The pins should sink
   fully into the breadboard holes.

The Arduino's pins land in **row d** (top half) and **row h** (bottom half). The board
body covers the center gap between them.

```
RESULT — Arduino in place:

  Columns:  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15
            ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓  ↓
    Row d:  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ← top pins (row d)
            ═══════════ CENTER GAP ════════════
    Row h:  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ■  ← bottom pins (row h)

  ■ = occupied hole (Arduino pin is here)
```

> **Tip:** If the pins are hard to push in, rock the board gently side to side while
> pressing. Never force it — you could bend a pin.

5. **Find and note your key pins.** The Arduino Nano ESP32 pin positions are listed
   below. Verify by reading the labels printed on your board (they may be on the back).

### Arduino Nano ESP32 — Complete Pin Map

**Top row (row d), USB-C on the left:**

| Column | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|--------|---|---|---|---|---|---|---|---|---|----|----|----|----|----|----|
| Pin    | D12 | D11 | D10 | D9 | D8 | D7 | D6 | D5 | D4 | D3 | D2 | **GND** | RST | RX0 | TX1 |

**Bottom row (row h), USB-C on the left:**

| Column | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|--------|---|---|---|---|---|---|---|---|---|----|----|----|----|----|----|
| Pin    | D13 | 3V3 | B0 | A0 | A1 | A2 | A3 | A4 | A5 | A6 | A7 | VBUS | B1 | **GND** | VIN |

**Key pins for this project:**

| Arduino Pin | Row | Column | Purpose |
|-------------|-----|--------|---------|
| VIN | h | 15 | 12V power input |
| GND (top) | d | 12 | Ground (use this one for power rail) |
| GND (bottom) | h | 14 | Ground (second GND pin) |
| D2 | d | 11 | Azimuth STEP signal |
| D3 | d | 10 | Azimuth DIR signal |
| D4 | d | 9 | Elevation STEP signal |
| D5 | d | 8 | Elevation DIR signal |
| A4 (SDA) | h | 8 | I2C data to RTC |
| A5 (SCL) | h | 9 | I2C clock to RTC |
| 3V3 | h | 2 | 3.3V output for RTC |

> **Note:** A4 and D5 share column 8, and A5 and D4 share column 9. This is normal —
> they are on opposite sides of the center gap and are NOT electrically connected.
> GND is denoted by a WHITE coloring around the pin hole on this board,rather than numbers. 

---

#### Step 1.2 — Place the Azimuth Motor Driver (E-04)

**What you're doing:** Seating the first TMC2209 driver on the breadboard.

1. Pick up the TMC2209 you labeled "AZ".
2. Orient it with the **red side facing up/toward row a** and the **black side facing
   down/toward row j**. The trimpot (tiny screw) should be at the EN/VS end.
3. It has two rows of 8 pins. These rows straddle the **center gap**, just like the Arduino.
4. Place it starting at **column 22**, red side in **row c**, black side in **row f**.
5. Press it gently into the breadboard.

```
RESULT — Azimuth driver in place:

  Columns:  22  23   24  25  26  27  28  29
            ↓   ↓    ↓   ↓   ↓   ↓   ↓   ↓
    Row c:  DIR STEP CLK TX  RX  MS2 MS1 EN   ← red side (signal/logic)
            ══════════ CENTER GAP ═════════════
    Row f:  GND VIO  B2  B1  A1  A2  GND VS   ← black side (power/motor)
```

### Azimuth Driver (E-04) — Complete Pin Map

**Red side (row c) — signal/logic pins:**

| Column | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 |
|--------|----|----|----|----|----|----|----|----|
| Pin    | DIR | STEP | CLK | TX | RX | MS2 | MS1 | EN |

**Black side (row f) — power/motor pins:**

| Column | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 |
|--------|----|----|----|----|----|----|----|----|
| Pin    | GND | VIO | B2 | B1 | A1 | A2 | GND | VS |

**Key pins for wiring:**

| Pin | Row | Column | Connect To |
|-----|-----|--------|------------|
| STEP | c | 23 | Arduino D2 (col 11) — green wire |
| DIR | c | 22 | Arduino D3 (col 10) — blue wire |
| EN | c | 29 | GND rail — gray wire (enables driver) |
| VIO | f | 23 | Top (+) rail, 3.3V — red wire |
| GND (logic) | f | 22 | (-) rail — black wire |
| VS | f | 29 | Bottom (+) rail, 12V — red wire |
| GND (motor) | f | 28 | Bottom (-) rail — black wire |
| A1 | f | 26 | Motor coil A wire |
| A2 | f | 27 | Motor coil A wire |
| B1 | f | 25 | Motor coil B wire |
| B2 | f | 24 | Motor coil B wire |

---

#### Step 1.3 — Place the Elevation Motor Driver (E-05)

Repeat Step 1.2 with the driver labeled "EL", starting at **column 36**. Same
orientation: red side in row c, black side in row f.

```
RESULT — Elevation driver in place:

  Columns:  36  37   38  39  40  41  42  43
            ↓   ↓    ↓   ↓   ↓   ↓   ↓   ↓
    Row c:  DIR STEP CLK TX  RX  MS2 MS1 EN   ← red side (signal/logic)
            ══════════ CENTER GAP ═════════════
    Row f:  GND VIO  B2  B1  A1  A2  GND VS   ← black side (power/motor)
```

### Elevation Driver (E-05) — Complete Pin Map

**Key pins for wiring:**

| Pin | Row | Column | Connect To |
|-----|-----|--------|------------|
| STEP | c | 37 | Arduino D4 (col 9) — green wire |
| DIR | c | 36 | Arduino D5 (col 8) — blue wire |
| EN | c | 43 | GND rail — gray wire (enables driver) |
| VIO | f | 37 | Top (+) rail, 3.3V — red wire |
| GND (logic) | f | 36 | (-) rail — black wire |
| VS | f | 43 | Bottom (+) rail, 12V — red wire |
| GND (motor) | f | 42 | Bottom (-) rail — black wire |
| A1 | f | 40 | Motor coil A wire |
| A2 | f | 41 | Motor coil A wire |
| B1 | f | 39 | Motor coil B wire |
| B2 | f | 38 | Motor coil B wire |

---

#### Step 1.4 — Place the RTC Module (E-06)

**What you're doing:** Seating the DS3231 clock module.

The RTC module has a **single row of pins** (not two rows like the Arduino/drivers). It
only goes into one side of the breadboard.

1. Hold the RTC so you can read the pin labels: **32K, SQW, SCL, SDA, VCC, GND**.
2. Plug the pin row into **row a** (the top row), starting at **column 52**.
3. The body of the RTC module will hang over the top power rail — this is fine.

```
RESULT — RTC in place:

  Columns:  52  53  54  55  56  57
            ↓   ↓   ↓   ↓   ↓   ↓
    Row a:  32K SQW SCL SDA VCC GND  ← RTC pins
    Row b:  ○   ○   ○   ○   ○   ○   ← available for wires
    Row c:  ○   ○   ○   ○   ○   ○   ← available for wires
    Row d:  ○   ○   ○   ○   ○   ○   ← available for wires
    Row e:  ○   ○   ○   ○   ○   ○   ← available for wires
```

### RTC Module (E-06) — Complete Pin Map

| Column | 52 | 53 | 54 | 55 | 56 | 57 |
|--------|----|----|----|----|----|----|
| Pin    | 32K | SQW | SCL | SDA | VCC | GND |

**Key pins for wiring:**

| Pin | Column | Connect To |
|-----|--------|------------|
| SCL | 54 (row b-e) | Arduino A5 (col 9) — yellow wire |
| SDA | 55 (row b-e) | Arduino A4 (col 8) — yellow wire |
| VCC | 56 (row b-e) | Top (+) rail, 3.3V — red wire |
| GND | 57 (row b-e) | Top (-) rail — black wire |

> **Note:** 32K and SQW are output pins we don't use in this project. Leave them unconnected.

---

#### Step 1.5 — Verify Component Placement

Before moving on, confirm:

- [x ] Arduino is in the breadboard, pins in rows d and h, USB port on the left, cols 1-15
- [ x] Azimuth driver (AZ) is in the breadboard, straddling the center gap, to the right of Arduino
- [x ] Elevation driver (EL) is in the breadboard, straddling the center gap, to the right of AZ driver
- [x ] RTC is in the breadboard, top half only, to the right of EL driver
- [x ] No components are touching each other
- [x ] You have verified the pin map matches the labels on your board
- [x ] Power is still disconnected

---

### Phase 2: Power Rails

**What you're doing:** Setting up the +12V and GND distribution lines that everything shares.

Think of power rails like the plumbing in a house — you set up the main water lines first,
then tap into them for each room.

---

#### Step 2.1 — Connect Ground (GND) from Arduino to Power Rail

**Wire:** Black
**From:** Any available hole in **column 12**, rows a-c (above the Arduino's top GND pin)
**To:** The bottom **(-) power rail**

```
Arduino GND is at column 12, row d:

    Row a: ○  ← use a12 to connect a wire
    Row b: ○
    Row c: ○  ← or c12
    Row d: ■  ← Arduino GND pin is here
    ─── gap ───
    Row h: ...

  Run a BLACK wire from a12 (or b12 or c12) down to the bottom (-) rail.
```

**Why:** This puts GND (0V reference) onto the power rail. Now anything else connected to
that (-) rail shares the same ground.

> **Note:** The Arduino has a second GND pin at **h14** (bottom row). You can use either
> one, but the top GND (d12) is more convenient since rows a-c are free above it.

---

#### Step 2.2 — Connect 12V from Arduino VIN to Power Rail

**Wire:** Red
**From:** Any available hole in **column 15**, rows i-j (below the Arduino's VIN pin at h15)
**To:** The bottom **(+) power rail**

> **Wait — don't we want 12V on the power rail?**
> Yes. The VIN pin will receive 12V from the power supply (we'll connect that later).
> By wiring VIN to the (+) rail now, we're preparing to distribute 12V to the drivers.
> Nothing will happen until we actually connect the power supply.

---

#### Step 2.3 — Bridge the Top and Bottom Power Rails

We need the top rails for the RTC and driver logic power. The four rails will carry:

| Rail | Voltage | Purpose |
|------|---------|---------|
| Bottom (+) | 12V | Motor power (from PSU via VIN) |
| Bottom (-) | GND (0V) | Ground (from Arduino GND) |
| Top (+) | 3.3V | Logic power for RTC and drivers |
| Top (-) | GND (0V) | Ground (bridged to bottom) |

> **CRITICAL: The top (+) rail and bottom (+) rail carry DIFFERENT voltages!**
> Bottom (+) = 12V (for motors). Top (+) = 3.3V (for RTC and driver logic).
> **Never connect them together.** Sending 12V to the RTC would destroy it instantly.

**Wire 1 — Bridge the ground rails:** Black
- **From:** Bottom (-) rail, any convenient column
- **To:** Top (-) rail, same or nearby column
- This makes GND available on both sides of the breadboard.

**Wire 2 — 3.3V to top (+) rail:** Red
- **From:** Column 2, row i or j (this connects to the Arduino's 3V3 pin at h2)
- **To:** Top (+) rail (above row a)
- This is a long wire — use a flexible jumper. It needs to reach from the bottom of
  the breadboard up to the top rail.

> **Why so far?** The 3V3 pin is on the bottom half (row h), but the top (+) rail is at
> the top edge. There's no shortcut — the wire has to span the full height of the board.

---

#### Step 2.4 — Verify Power Rails

Your power rail connections should now look like this:

```
  TOP (+) ═══ 3.3V ═══════════════════════════════════════  (from Arduino 3V3 at col 2)
  TOP (-) ═══ GND  ═══════════════════════════════════════  (bridged to bottom)

  ... breadboard holes ...

  BOT (+) ═══ 12V  ═══════════════════════════════════════  (will come from PSU via VIN at col 15)
  BOT (-) ═══ GND  ═══════════════════════════════════════  (from Arduino GND at col 12)
```

Checklist:
- [x ] Arduino GND (col 12, row a-c) → bottom (-) rail — **black wire**
- [x ] Arduino VIN (col 15, row i-j) → bottom (+) rail — **red wire**
- [x ] Bottom (-) bridged to Top (-) — **black wire**
- [x ] Arduino 3V3 (col 2, row i-j) → Top (+) rail — **red wire**
- [x ] Top (+) is **NOT** bridged to bottom (+)

---

### Phase 3: Power the Motor Drivers

**What you're doing:** Giving each TMC2209 driver its power supply connections.

Each driver needs TWO types of power:
1. **VS (motor voltage):** 12V from the bottom (+) rail — the heavy power that spins the motor
2. **VIO (logic voltage):** 3.3V from the top (+) rail — the gentle power that lets the driver think

Both VS and VIO are on the **black side** (row f), so all four power wires connect
through **rows g-j** (which are electrically connected to row f in the same column).

---

#### Step 3.1 — Azimuth Driver Power (E-04)

**Wire 1 (12V to motor power):** Red
- **From:** Bottom (+) rail (12V)
- **To:** **Column 29**, rows g-j (connects to AZ driver **VS** at f29)

**Wire 2 (GND for motor side):** Black
- **From:** Bottom (-) rail (GND)
- **To:** **Column 28**, rows g-j (connects to AZ driver **GND** at f28)

**Wire 3 (3.3V to logic power):** Red
- **From:** Top (+) rail (3.3V)
- **To:** **Column 23**, rows g-j (connects to AZ driver **VIO** at f23)
- This is a longer wire — it runs from the top edge of the board down to the bottom half.

**Wire 4 (GND for logic side):** Black
- **From:** Bottom (-) rail (GND)
- **To:** **Column 22**, rows g-j (connects to AZ driver **GND** at f22) (bottom or top)

---

#### Step 3.2 — Elevation Driver Power (E-05)

Repeat Step 3.1 exactly, but for the EL driver at columns 36-43.

- VS (col 43, row g-j) → bottom (+) rail (12V) — **red wire**
- GND motor (col 42, row g-j) → bottom (-) rail — **black wire**
- VIO (col 37, row g-j) → top (+) rail (3.3V) — **red wire**
- GND logic (col 36, row g-j) → (-) rail (bottom or top)— **black wire**

---

#### Step 3.3 — Verify Driver Power

Checklist:
- [x ] AZ driver: VS (col 29) connected to 12V rail — **red**
- [x ] AZ driver: GND motor (col 28) connected to GND rail — **black**
- [x ] AZ driver: VIO (col 23) connected to 3.3V rail — **red**
- [x ] AZ driver: GND logic (col 22) connected to GND rail — **black**
- [x ] EL driver: VS (col 43) connected to 12V rail — **red**
- [x ] EL driver: GND motor (col 42) connected to GND rail — **black**
- [x ] EL driver: VIO (col 37) connected to 3.3V rail — **red**
- [x ] EL driver: GND logic (col 36) connected to GND rail — **black**

---

### Phase 4: Signal Wires (STEP and DIR)

**What you're doing:** Connecting the Arduino's brain to the motor drivers so it can tell
them when and which direction to spin.

Each motor driver needs exactly 2 signals from the Arduino:
- **STEP:** Every electrical pulse on this wire = one tiny step of the motor
- **DIR:** HIGH voltage = clockwise, LOW voltage = counterclockwise (or vice versa)

---

#### Step 4.1 — Azimuth STEP (Arduino D2 → AZ Driver STEP)

**Wire:** Green (flexible jumper wire — this is a longer run)

1. Arduino pin **D2** is at **column 11, row d**.
2. AZ driver pin **STEP** is at **column 23, row c**.
3. Plug one end of a **green** flexible jumper wire into an available hole in
   **column 11**, rows a-b (above the D2 pin).
4. Plug the other end into an available hole in **column 23**, rows a-b (above the STEP pin).

> **Remember the breadboard rule:** any hole in the same column AND same side of the gap
> is electrically connected to the pin. So you don't have to plug the wire into the pin's
> exact hole — any hole that shares the column (and side) works.

---

#### Step 4.2 — Azimuth DIR (Arduino D3 → AZ Driver DIR)

**Wire:** Blue (flexible jumper wire)

Same process:
- One end in **column 10**, rows a-b (Arduino D3 is at d10)
- Other end in **column 22**, rows a-b (AZ driver DIR is at c22)

---

#### Step 4.3 — Elevation STEP (Arduino D4 → EL Driver STEP)

**Wire:** Green (flexible jumper wire)

- One end in **column 9**, rows a-b (Arduino D4 is at d9)
- Other end in **column 37**, rows a-b (EL driver STEP is at c37)

---

#### Step 4.4 — Elevation DIR (Arduino D5 → EL Driver DIR)

**Wire:** Blue (flexible jumper wire)

- One end in **column 8**, rows a-b (Arduino D5 is at d8)
- Other end in **column 36**, rows a-b (EL driver DIR is at c36)

---

#### Step 4.5 — Verify Signal Wires

Checklist:
- [x ] Arduino D2 → AZ Driver STEP — **green wire**
- [x ] Arduino D3 → AZ Driver DIR — **blue wire**
- [x ] Arduino D4 → EL Driver STEP — **green wire**
- [x ] Arduino D5 → EL Driver DIR — **blue wire**

You should now see 4 wires running from the Arduino area to the driver area:
2 green, 2 blue.

---

### Phase 5: Wire the RTC Module

**What you're doing:** Connecting the clock module so the Arduino can ask it "what time
is it?" over the I2C communication bus.

I2C uses just 2 wires (plus power and ground):
- **SDA (data):** Carries the actual time information back and forth
- **SCL (clock):** A timing signal that keeps communication synchronized

---

#### Step 5.1 — RTC Power

**Wire 1 (VCC):** Red
- **From:** Top (+) rail (3.3V)
- **To:** Available hole in **column 56**, rows b-e (RTC VCC is at a56)

**Wire 2 (GND):** Black
- **From:** Top (-) rail (GND)
- **To:** Available hole in **column 57**, rows b-e (RTC GND is at a57)

--

#### Step 5.2 — RTC Data (SDA)

**Wire:** Yellow (flexible jumper wire)

- **From:** Available hole in **column 8**, rows i-j (Arduino A4/SDA is at h8)
- **To:** Available hole in **column 55**, rows b-e (RTC SDA is at a55)

---

#### Step 5.3 — RTC Clock (SCL)

**Wire:** Yellow (flexible jumper wire)

- **From:** Available hole in **column 9**, rows i-j (Arduino A5/SCL is at h9)
- **To:** Available hole in **column 54**, rows b-e (RTC SCL is at a54)

---

#### Step 5.4 — Verify RTC Wiring

Checklist:
- [x ] RTC VCC → 3.3V rail (top +) — **red wire**
- [x ] RTC GND → GND rail (top -) — **black wire**
- [x ] Arduino A4 → RTC SDA — **yellow wire**
- [x ] Arduino A5 → RTC SCL — **yellow wire**

---

### Phase 6: Motor Coil Wires

**What you're doing:** Connecting the stepper motors to the driver boards.

Each stepper motor has **4 wires** (Red, Green, Blue, Black). These form **2 coil pairs**:
- **Coil A:** 2 wires
- **Coil B:** 2 wires

The driver has 4 output pins for these: **A1, A2** (coil A) and **B1, B2** (coil B).

> **IMPORTANT:** Getting the coil pairs right matters. If you swap wires between coils,
> the motor will vibrate instead of spinning smoothly. But it won't break anything — you
> can just swap wires until it works.

---

#### Step 6.1 — Identify Motor Wire Pairs

Your 42HB48-49R motors have 4 wires: Red, Green, Blue, Black.

Typical bipolar stepper wire pairing (verify with your motor's datasheet or by testing):

| Driver Pin | Motor Wire | Coil |
|------------|------------|------|
| A1 | Red | A |
| A2 | Blue | A |
| B1 | Green | B |
| B2 | Black | B |

> **If you don't know the pairing:** Touch two wires together and try to turn the motor
> shaft by hand. If the shaft feels harder to turn, those two wires are a coil pair.
> If the shaft turns freely, they're from different coils — try a different combination.
> There are only 3 possible pairings to try.

---

#### Step 6.2 — Connect Azimuth Motor (E-01) to Azimuth Driver (E-04)

The motor wires won't plug directly into the breadboard (they're the wrong gauge). Use
**F/M dupont wires** to adapt:

1. Connect a **female** dupont connector to the motor wire
2. Plug the **male** end into the breadboard at the driver's output column

| Connection | From (Motor Wire) | To (AZ Driver Pin) | Column | Dupont Wire Color |
|------------|-------------------|---------------------|--------|-------------------|
| Coil A+ | Red motor wire | A1 | 26 (row g-j) | Orange |
| Coil A- | Blue motor wire | A2 | 27 (row g-j) | Orange |
| Coil B+ | Green motor wire | B1 | 25 (row g-j) | Yellow |
| Coil B- | Black motor wire | B2 | 24 (row g-j) | Yellow |

> **Alternative:** If your motor has a connector that matches the dupont header cables,
> you can use the F/F cables to bridge them. The exact method depends on your motor's
> connector type.

---

#### Step 6.3 — Connect Elevation Motor (E-02) to Elevation Driver (E-05)

Repeat Step 6.2 for the elevation motor and driver.

| Connection | From (Motor Wire) | To (EL Driver Pin) | Column | Dupont Wire Color |
|------------|-------------------|---------------------|--------|-------------------|
| Coil A+ | Red motor wire | A1 | 40 (row g-j) | Orange |
| Coil A- | Blue motor wire | A2 | 41 (row g-j) | Orange |
| Coil B+ | Green motor wire | B1 | 39 (row g-j) | Yellow |
| Coil B- | Black motor wire | B2 | 38 (row g-j) | Yellow |

---

#### Step 6.4 — Verify Motor Wiring

Checklist:
- [ ] AZ motor: 4 wires connected to AZ driver (A1, A2, B1, B2)
- [ ] EL motor: 4 wires connected to EL driver (A1, A2, B1, B2)
- [ ] Orange wires for coil A pairs, Yellow for coil B pairs
- [ ] Wires are secure, not pulling on the breadboard

---

### Phase 7: Driver Enable

**What you're doing:** Making sure the motor drivers are turned ON and ready to accept
commands.

The TMC2209 has an **EN (Enable)** pin:
- **EN pulled LOW (connected to GND):** Driver is ENABLED (motor can move)
- **EN left floating or pulled HIGH:** Driver is DISABLED

---

#### Step 7.1 — Enable the Azimuth Driver

**Wire:** Gray or any color (short U-shape jumper)

- **From:** Available hole in **column 29**, rows a-b (AZ driver EN is at c29)
- **To:** Any nearby (-) GND rail (top rail is closest)

This permanently enables the driver. (Later in software, you can control this via a GPIO
pin for power saving, but for initial testing, just tie it to GND.)

---

#### Step 7.2 — Enable the Elevation Driver

Same thing for the EL driver:

- **From:** Available hole in **column 43**, rows a-b (EL driver EN is at c43)
- **To:** (-) GND rail

---

### Phase 8: Decoupling Capacitors (When They Arrive)

> **You can skip this phase for now** — capacitors (E-11) are on order. Come back and
> add them when they arrive. The system will work without them, but may be less stable.

**What you're doing:** Adding a small energy reservoir next to each motor driver to smooth
out voltage spikes when the motors start and stop.

**What a capacitor looks like:**

```
    ┌──┐
    │  │  ← small cylinder with two wire legs
    │  │     ONE LEG IS LONGER — that's the (+) positive leg
    └──┘     The short leg (or the side with a stripe) is (-) negative
    │  │
    ↓  ↓
   (+) (-)
```

> **IMPORTANT:** Electrolytic capacitors have a polarity — the (+) and (-) matter!
> Putting it in backwards can cause the capacitor to pop (literally).

---

#### Step 8.1 — Capacitor for Azimuth Driver

1. AZ driver's **VS** is at column 29, **GND** (motor side) is at column 28 — right next to each other.
2. Push the capacitor's **(+) long leg** into an available hole in **column 29**, rows g-j.
3. Push the capacitor's **(-) short leg** into an available hole in **column 28**, rows g-j.

The capacitor should sit right next to the driver, bridging VS to GND.

---

#### Step 8.2 — Capacitor for Elevation Driver

Repeat for the EL driver:
- (+) leg → **column 43**, rows g-j (VS)
- (-) leg → **column 42**, rows g-j (GND)

---

### Phase 8.5 — Connect the 12V Power Supply Wires

**What you're doing:** Connecting the PSU's bare wires to the breadboard so 12V reaches
the VIN pin and motor drivers.

1. Take the **red (+12V)** wire from the power supply.
2. Strip ~5mm of insulation from the end if not already stripped.
3. Plug it into the **bottom (+) power rail** (the 12V rail).

   > If the bare wire is too thick for the breadboard hole, you have options:
   > - Use the quick-connect adapter that came with the PSU, then attach a male jumper wire
   > - Twist the stranded wire tight and tin it with solder to make it stiff enough
   > - Use a screw terminal breadboard adapter (cheap, ~$2)

4. Take the **black (GND)** wire from the power supply.
5. Plug it into the **bottom (-) power rail** (the GND rail).

> **Do NOT plug the power supply into the wall yet!** We still need to verify everything.

---

## 8. Final Checklist Before Powering On

Go through every single item. **Do not skip any.**

### Power Connections

| # | Check | OK? |
|---|-------|-----|
| 1 | PSU red wire (+12V) → bottom (+) rail | [ ] |
| 2 | PSU black wire (GND) → bottom (-) rail | [ ] |
| 3 | Bottom (+) rail → Arduino VIN (col 15) | [ ] |
| 4 | Arduino GND (col 12) → bottom (-) rail | [ ] |
| 5 | Bottom (-) rail bridged to top (-) rail | [ ] |
| 6 | Arduino 3V3 (col 2) → top (+) rail | [ ] |
| 7 | Top (+) rail is **NOT** connected to bottom (+) rail | [ ] |

### Azimuth Driver (E-04)

| # | Check | OK? |
|---|-------|-----|
| 8 | VS (col 29) → bottom (+) rail (12V) | [ ] |
| 9 | GND motor (col 28) → bottom (-) rail | [ ] |
| 10 | VIO (col 23) → top (+) rail (3.3V) | [ ] |
| 11 | GND logic (col 22) → (-) rail | [ ] |
| 12 | EN (col 29) → GND (enabled) | [ ] |
| 13 | STEP (col 23) ← Arduino D2, col 11 (green wire) | [ ] |
| 14 | DIR (col 22) ← Arduino D3, col 10 (blue wire) | [ ] |
| 15 | A1 (col 26) → Motor red wire | [ ] |
| 16 | A2 (col 27) → Motor blue wire | [ ] |
| 17 | B1 (col 25) → Motor green wire | [ ] |
| 18 | B2 (col 24) → Motor black wire | [ ] |

### Elevation Driver (E-05)

| # | Check | OK? |
|---|-------|-----|
| 19 | VS (col 43) → bottom (+) rail (12V) | [ ] |
| 20 | GND motor (col 42) → bottom (-) rail | [ ] |
| 21 | VIO (col 37) → top (+) rail (3.3V) | [ ] |
| 22 | GND logic (col 36) → (-) rail | [ ] |
| 23 | EN (col 43) → GND (enabled) | [ ] |
| 24 | STEP (col 37) ← Arduino D4, col 9 (green wire) | [ ] |
| 25 | DIR (col 36) ← Arduino D5, col 8 (blue wire) | [ ] |
| 26 | A1 (col 40) → Motor red wire | [ ] |
| 27 | A2 (col 41) → Motor blue wire | [ ] |
| 28 | B1 (col 39) → Motor green wire | [ ] |
| 29 | B2 (col 38) → Motor black wire | [ ] |

### RTC Module (E-06)

| # | Check | OK? |
|---|-------|-----|
| 30 | VCC (col 56) → top (+) rail (3.3V) | [ ] |
| 31 | GND (col 57) → top (-) rail | [ ] |
| 32 | SDA (col 55) ← Arduino A4, col 8 (yellow wire) | [ ] |
| 33 | SCL (col 54) ← Arduino A5, col 9 (yellow wire) | [ ] |

### Critical Safety Checks

| # | Check | OK? |
|---|-------|-----|
| 34 | **No bare wires touching each other** | [ ] |
| 35 | **No wire running from (+) rail directly to (-) rail** (short circuit!) | [ ] |
| 36 | **12V rail is NOT connected to 3.3V rail** | [ ] |
| 37 | **All components seated firmly** — none popping out | [ ] |
| 38 | **Motor shafts are clear** — nothing will get caught if they spin | [ ] |

---

## 9. First Power-On

### Step 9.1 — USB Only First (No 12V)

1. **Leave the 12V power supply UNPLUGGED from the wall.**
2. Connect the USB-C cable from your computer to the Arduino.
3. The Arduino should power on — look for a small LED on the board lighting up.
4. If you've already flashed firmware with the heartbeat blink, the LED on GPIO 48
   should start blinking.

**If nothing happens:**
- Check the USB cable is fully inserted
- Try a different USB port
- Verify the Arduino is seated properly in the breadboard

**If you smell burning or see smoke:** Unplug USB immediately. Something is wired wrong.
Go back to the checklist.

### Step 9.2 — Add 12V Power

1. Verify USB is connected and Arduino is working from Step 9.1.
2. Plug the 12V power supply into the wall outlet.
3. **Watch the breadboard for 3 seconds.** Look for:
   - Smoke (bad — unplug everything immediately)
   - Hot components (bad — unplug everything, find the short)
   - Nothing dramatic (good!)
4. Gently touch the TMC2209 heatsinks — they should be warm, not hot.

### Step 9.3 — Test Motor Movement

Once firmware with motor control is flashed, you can test:
- The motors should hold position (resist being turned by hand) when enabled
- Sending STEP pulses from firmware should cause smooth, quiet rotation
- If a motor vibrates but doesn't spin, swap one pair of motor wires (e.g., swap 1A and 1B)

---

## 10. Troubleshooting

### "Nothing happens when I power on"

1. Is the USB cable a data cable? (Some USB-C cables are charge-only)
2. Is the Arduino seated fully in the breadboard? Press it down firmly.
3. Check your GND wire — if ground isn't connected, nothing works.

### "The motor vibrates but doesn't spin"

The motor wire pairs are swapped. Try these combinations:

| Attempt | A1 | A2 | B1 | B2 |
|---------|----|----|----|----|
| Try 1 (original) | Red | Blue | Green | Black |
| Try 2 | Red | Green | Blue | Black |
| Try 3 | Red | Black | Green | Blue |

One of these will work. You can't break anything by trying.

### "The motor spins the wrong direction"

Swap the DIR signal — either in software (invert the pin), or physically swap the two
wires of ONE coil pair (e.g., swap A1 and A2).

### "The motor gets very hot"

The TMC2209 current is set too high. Turn the **trimpot** (small screw on the driver)
counterclockwise with a tiny screwdriver to reduce current. Your motors are rated for
1.5A, so set the driver to ~1.0-1.2A.

### "The Arduino resets when motors move"

The motors are causing voltage drops that reset the Arduino. This is exactly what the
**decoupling capacitors** (E-11) fix. Install them when they arrive. As a temporary fix,
power the Arduino via USB instead of through VIN.

### "I2C / RTC not responding"

1. Check SDA and SCL wires aren't swapped
2. Verify RTC has a battery installed (small coin cell on the module)
3. Confirm VCC is 3.3V, not 12V (would destroy the RTC)

---

## 11. Complete Wiring Reference Table

Every single wire in the project:

| # | From | To | Wire Color | Length | Purpose |
|---|------|----|------------|--------|---------|
| 1 | PSU Red (+12V) | Bottom (+) rail | Red | — | 12V supply input |
| 2 | PSU Black (GND) | Bottom (-) rail | Black | — | Ground supply input |
| 3 | Arduino VIN (col 15) | Bottom (+) rail | Red | Short | 12V to Arduino |
| 4 | Arduino GND (col 12) | Bottom (-) rail | Black | Short | Ground reference |
| 5 | Arduino 3V3 (col 2) | Top (+) rail | Red | Short | 3.3V distribution |
| 6 | Bottom (-) rail | Top (-) rail | Black | Short | Bridge ground rails |
| 7 | Bottom (+) rail | AZ VS (col 29) | Red | Medium | 12V to AZ motor power |
| 8 | Bottom (-) rail | AZ GND motor (col 28) | Black | Medium | AZ motor ground |
| 9 | Top (+) rail | AZ VIO (col 23) | Red | Medium | 3.3V to AZ logic |
| 10 | (-) rail | AZ GND logic (col 22) | Black | Short | AZ logic ground |
| 11 | (-) rail | AZ EN (col 29) | Gray | Short | Enable AZ driver |
| 12 | Bottom (+) rail | EL VS (col 43) | Red | Medium | 12V to EL motor power |
| 13 | Bottom (-) rail | EL GND motor (col 42) | Black | Medium | EL motor ground |
| 14 | Top (+) rail | EL VIO (col 37) | Red | Medium | 3.3V to EL logic |
| 15 | (-) rail | EL GND logic (col 36) | Black | Short | EL logic ground |
| 16 | (-) rail | EL EN (col 43) | Gray | Short | Enable EL driver |
| 17 | Arduino D2 (col 11) | AZ STEP (col 23) | Green | Long | Azimuth step signal |
| 18 | Arduino D3 (col 10) | AZ DIR (col 22) | Blue | Long | Azimuth direction signal |
| 19 | Arduino D4 (col 9) | EL STEP (col 37) | Green | Long | Elevation step signal |
| 20 | Arduino D5 (col 8) | EL DIR (col 36) | Blue | Long | Elevation direction signal |
| 21 | Top (+) rail | RTC VCC (col 56) | Red | Short | 3.3V to RTC |
| 22 | Top (-) rail | RTC GND (col 57) | Black | Short | RTC ground |
| 23 | Arduino A4 (col 8) | RTC SDA (col 55) | Yellow | Long | I2C data |
| 24 | Arduino A5 (col 9) | RTC SCL (col 54) | Yellow | Long | I2C clock |
| 25 | AZ A1 (col 26) | AZ Motor Red wire | Orange | — | AZ motor coil A+ |
| 26 | AZ A2 (col 27) | AZ Motor Blue wire | Orange | — | AZ motor coil A- |
| 27 | AZ B1 (col 25) | AZ Motor Green wire | Yellow | — | AZ motor coil B+ |
| 28 | AZ B2 (col 24) | AZ Motor Black wire | Yellow | — | AZ motor coil B- |
| 29 | EL A1 (col 40) | EL Motor Red wire | Orange | — | EL motor coil A+ |
| 30 | EL A2 (col 41) | EL Motor Blue wire | Orange | — | EL motor coil A- |
| 31 | EL B1 (col 39) | EL Motor Green wire | Yellow | — | EL motor coil B+ |
| 32 | EL B2 (col 38) | EL Motor Black wire | Yellow | — | EL motor coil B- |

**Total wire count: 32 connections** (+ 2 capacitors when E-11 arrives)

---

## Congratulations!

If you've made it here with all checkboxes ticked, your heliostat electronics are wired
and ready for firmware. The next step is flashing the Arduino Nano ESP32 — see
`first-flash-guide.md` for that procedure.

```
What you built:

  ┌──────────┐    STEP/DIR     ┌──────────┐    4 wires    ╔══════════╗
  │          │───────(green)──→│  TMC2209  │──────────────→║  MOTOR   ║
  │ Arduino  │───────(blue)──→│ (Azimuth) │               ║ (Azimuth)║
  │ Nano     │                 └──────────┘                ╚══════════╝
  │ ESP32    │
  │          │    STEP/DIR     ┌──────────┐    4 wires    ╔════════════╗
  │          │───────(green)──→│  TMC2209  │──────────────→║   MOTOR    ║
  │          │───────(blue)──→│(Elevation)│               ║(Elevation) ║
  │          │                 └──────────┘                ╚════════════╝
  │          │
  │          │    SDA/SCL      ┌──────────┐
  │          │───(yellow)─────→│  DS3231  │
  │          │───(yellow)─────→│   RTC    │
  └──────────┘                 └──────────┘
       │
    [USB-C]
    to computer
```
