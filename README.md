# AutoKeyboard

An ESP32-S3 keyboard firmware built on [RMK](https://github.com/HaoboGu/rmk), written in Rust (`no_std`).

On top of a standard USB HID keyboard (6×17 matrix, 2 keymap layers, Vial remapping), it adds **WiFi + MQTT remote key triggering**: once the device is online it subscribes to an MQTT topic, and an incoming message can simulate pressing/releasing a given key — useful for automation, remote demos, and similar scenarios.

## Features

- **USB HID keyboard**: runs on the USB OTG stack, plug-and-play with no drivers
- **Vial support**: remap keys online with the Vial client; configuration is stored in Flash
- **6×17 matrix**: `ROW = 6`, `COL = 17`, with 2 default layers (layer 0 is the base layer, `mo!(1)` activates layer 1)
- **Debouncing and WPM statistics**: uses RMK's built-in `DefaultDebouncer` and `WpmProcessor`
- **Automatic WiFi connection**: periodically scans for the target SSID on boot and obtains an IP via DHCP once connected
- **MQTT remote keys**: connects to the broker and subscribes to the topic automatically, then triggers key events from incoming payloads
- **Automatic reconnection**: retries WiFi every 5s when disconnected, reconnects MQTT every 10s on error

## Hardware

| Item | Description |
| --- | --- |
| Chip | ESP32-S3 (4MB Flash) |
| Matrix rows (inputs, pull-up) | GPIO15, GPIO18, GPIO21, GPIO35, GPIO36, GPIO38 |
| Matrix columns (outputs) | GPIO10, GPIO9, GPIO8, GPIO7, GPIO6, GPIO5, GPIO4, GPIO2, GPIO11, GPIO12, GPIO13, GPIO14, GPIO16, GPIO17, GPIO34, GPIO37, GPIO39 |
| USB | USB0 (D- = GPIO19, D+ = GPIO20) |
| Storage region | 16 sectors starting at Flash `0x3f0000` (holds keymap and other configuration) |

USB device information: VID `0x4c4b`, PID `0x4643`, product name `AIKeyboard`.

## Prerequisites

Install the **esp** channel of the Rust toolchain; see the [esp-rs book](https://docs.esp-rs.org/book/installation/index.html) for full instructions.

You also need [`espflash`](https://github.com/esp-rs/espflash):

```bash
cargo install cargo-espflash espflash
```

The toolchain and target are already configured in the repository, so no manual selection is needed:

- `rust-toolchain.toml`: `channel = "esp"`
- `.cargo/config.toml`: `target = "xtensa-esp32s3-none-elf"`, with `espflash flash --monitor` as the runner

## Building and Flashing

After the first clone you must create the WiFi credentials file, otherwise `build.rs` fails immediately (see "WiFi Credentials" below):

```bash
cp src/wifi_credentials.rs.example src/wifi_credentials.rs
# then edit src/wifi_credentials.rs and fill in the real SSID and password
```

```bash
# Build, flash, and open the serial monitor
cargo run --release

# Specify the serial port (auto-detected by default; use this with multiple devices)
cargo run --release -- --port COM5
```

The build artifact is placed at `target/xtensa-esp32s3-none-elf/release/AutoKeyboard`.

If `espflash` reports `Serial port not found`, confirm the port number with `espflash board-info` or Device Manager, then pass it via `--port`.

To inspect section sizes of the firmware (optional):

```bash
cargo install --git https://github.com/bjoernQ/espsegs
espsegs target/xtensa-esp32s3-none-elf/release/AutoKeyboard --chip esp32s3
```

The log level is controlled by an environment variable and defaults to `ESP_LOG=info`, set in `.cargo/config.toml`.

## Configuration

### 1. WiFi Credentials

Credentials live in `src/wifi_credentials.rs`, which is **ignored by `.gitignore`** and never enters the repository. `src/main.rs` pulls it in via `include!`, so the values are inlined at compile time (no runtime file is read):

```rust
// src/wifi_credentials.rs (local file, do not commit)
const WIFI_SSID: &str = "your-ssid";
const WIFI_PASSWORD: &str = "your-password";
```

A fresh clone only contains the template `src/wifi_credentials.rs.example`; copy it and fill it in:

```bash
cp src/wifi_credentials.rs.example src/wifi_credentials.rs
```

If the file is missing, `build.rs` prints a hint and aborts at the start of the build, instead of raising a cryptic `include!` error.

The device only connects to an AP whose scan result exactly matches `WIFI_SSID`.

### 2. MQTT Broker

Modify `src/mqtt.rs`:

```rust
pub const MQTT_BROKER: &str = "192.168.3.15"; // Broker IP (only IPv4 literals are supported)
pub const MQTT_PORT: u16 = 1883;
pub const MQTT_CLIENT_ID: &str = "AutoKeyboard";
pub const MQTT_TOPIC: &str = "keyboard/auto";
```

> When multiple devices are connected at the same time, change `MQTT_CLIENT_ID` to avoid client ID collisions kicking each other offline.

### 3. Matrix Pins

Configured in `src/main.rs` via the `config_matrix_pins_esp!` macro:

```rust
let (row_pins, col_pins) = config_matrix_pins_esp!(
    peripherals: peripherals,
    input:  [GPIO15, GPIO18, GPIO21, GPIO35, GPIO36, GPIO38],
    output: [GPIO10, ..., GPIO39]
);
```

The row/column counts come from the `ROW` and `COL` constants in `src/keymap.rs`; change them in sync when you change the number of pins.

### 4. Keymaps

- **Vial remapping**: connect with the Vial client and remap directly; the configuration is persisted to Flash
- **Default keymap**: `get_default_keymap()` in `src/keymap.rs`, only takes effect when Flash holds no valid configuration
- **Vial layout description**: `vial.json` in the project root; `build.rs` compresses it at build time and generates `VIAL_KEYBOARD_DEF`. Note that `keyboard_id` in `build.rs` must match the firmware, otherwise Vial will not recognize the device

To restore the factory keymap after flashing, erase the configuration sectors (`espflash erase-flash`) and flash again.

## MQTT Remote Key Protocol

Subscribed topic: `keyboard/auto`

The payload must be at least 10 bytes, in little-endian order:

| Offset | Length | Field | Description |
| --- | --- | --- | --- |
| 0 | 8 | `msg_id` | `u64` message ID, little-endian |
| 8 | 1 | `key_code` | HID KeyCode, matching `key_code` in `KEY_MAPPING` |
| 9 | 1 | `pressed` | `0` = release, non-zero = press |

On receiving a message, the firmware looks up the matrix coordinate `(row, col)` for the HID key value in the `KEY_MAPPING` table in `src/keymap.rs`, then injects a key event through `rmk::event::publish_event_async`. Key values not registered in the table log an error and are ignored.

`msg_id` currently only appears in logs; no deduplication is performed.

### Example Sender (Python)

```python
import struct, paho.mqtt.client as mqtt

# HID KeyCode 'A' = 0x04, arrow up = 0x52
payload = struct.pack("<QBB", msg_id=1, key_code=0x04, pressed=1)

client = mqtt.Client()
client.connect("192.168.3.15", 1883)
client.publish("keyboard/auto", payload, qos=0)
```

## Project Structure

```
.
├── build.rs          # Validates that the WiFi credentials file exists and generates the Vial config
├── vial.json         # Vial layout description (matrix dimensions, physical layout)
├── Cargo.toml        # Dependencies and build profiles
└── src/
    ├── main.rs       # Entry point: peripheral/USB/Flash init, RMK assembly, WiFi task
    ├── keymap.rs     # Matrix dimensions, HID key value ↔ matrix coordinate table, default keymap
    ├── mqtt.rs       # MQTT client task: connect, subscribe, parse payloads, inject key events
    ├── vial.rs       # Includes the Vial config generated by build.rs
    ├── macros.rs     # Macros such as matrix pin configuration
    ├── wifi_credentials.rs.example  # WiFi credentials template (checked into the repo)
    └── wifi_credentials.rs          # Real credentials (local file, ignored by .gitignore)
```

## Dependencies

- [`rmk`](https://github.com/HaoboGu/rmk): keyboard firmware framework (with the `esp32s3_ble`, `log`, `storage`, and `vial` features)
- [`esp-hal`](https://github.com/esp-rs/esp-hal) / `esp-radio` / `esp-rtos`: ESP32-S3 hardware abstraction and runtime
- `embassy-executor` / `embassy-net`: async runtime and network stack
- `rust-mqtt`: MQTT v5 client (`no_std`)

## Roadmap

- [ ] Make WiFi credentials and MQTT configuration runtime-configurable (via Vial or the storage region); credentials are already out of the repository, but they are still inlined at compile time, so switching networks requires reflashing
- [ ] Add `msg_id` deduplication and QoS 1 support for MQTT payloads
- [ ] Complete the Vial layout description to match the actual 6×17 keyboard
- [ ] Add a BLE wireless mode (related code is kept commented out in `main.rs`)
