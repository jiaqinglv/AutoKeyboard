# AutoKeyboard

基于 [RMK](https://github.com/HaoboGu/rmk) 的 ESP32-S3 键盘固件，使用 Rust (`no_std`) 编写。

在标准 USB HID 键盘（6×17 矩阵、2 层键位、Vial 可视化改键）的基础上，增加了 **WiFi + MQTT 远程触发按键** 的能力：设备联网后订阅 MQTT 主题，收到消息即可模拟按下/松开指定按键，用于自动化、远程演示等场景。

## 功能特性

- **USB HID 键盘**：走 USB OTG 协议栈，免驱即插即用
- **Vial 支持**：可通过 Vial 客户端在线修改键位，配置保存在 Flash 中
- **6×17 矩阵**：`ROW = 6`、`COL = 17`，默认 2 层键位（1 层为基础层，`mo!(1)` 进入第 2 层）
- **按键消抖与 WPM 统计**：使用 RMK 内置 `DefaultDebouncer` 与 `WpmProcessor`
- **WiFi 自动连接**：启动后周期扫描目标 SSID，连接成功后通过 DHCP 获取 IP
- **MQTT 远程按键**：联网后自动连接 Broker 并订阅主题，支持通过报文触发按键事件
- **断线自动重连**：WiFi 断开每 5s 重试，MQTT 异常每 10s 重连

## 硬件

| 项目 | 说明 |
| --- | --- |
| 芯片 | ESP32-S3（Flash 4MB） |
| 矩阵行（输入，带上拉） | GPIO15, GPIO18, GPIO21, GPIO35, GPIO36, GPIO38 |
| 矩阵列（输出） | GPIO10, GPIO9, GPIO8, GPIO7, GPIO6, GPIO5, GPIO4, GPIO2, GPIO11, GPIO12, GPIO13, GPIO14, GPIO16, GPIO17, GPIO34, GPIO37, GPIO39 |
| USB | USB0（D- = GPIO19，D+ = GPIO20） |
| 存储区 | Flash `0x3f0000` 起 16 个扇区（存放键位等配置） |

USB 设备信息：VID `0x4c4b`、PID `0x4643`，产品名 `AIKeyboard`。

## 环境准备

需要安装 **esp** 渠道的 Rust 工具链，完整说明见 [esp-rs book](https://docs.esp-rs.org/book/installation/index.html)。

另外需要安装 [`espflash`](https://github.com/esp-rs/espflash)：

```bash
cargo install cargo-espflash espflash
```

工具链与目标已在仓库中配置好，无需手动指定：

- `rust-toolchain.toml`：`channel = "esp"`
- `.cargo/config.toml`：`target = "xtensa-esp32s3-none-elf"`，并用 `espflash flash --monitor` 作为 runner

## 构建与烧录

```bash
# 编译并烧录、打开串口监视器
cargo run --release

# 指定串口（默认自动探测，多设备时使用）
cargo run --release -- --port COM5
```

编译产物位于 `target/xtensa-esp32s3-none-elf/release/AutoKeyboard`。

如果 `espflash` 报错 `Serial port not found`，先用 `espflash board-info` 或设备管理器确认串口号，再通过 `--port` 指定。

查看固件各段大小（可选）：

```bash
cargo install --git https://github.com/bjoernQ/espsegs
espsegs target/xtensa-esp32s3-none-elf/release/AutoKeyboard --chip esp32s3
```

日志级别通过环境变量控制，默认在 `.cargo/config.toml` 中设置为 `ESP_LOG=info`。

## 配置

### 1. WiFi 账号密码

`src/main.rs` 中的常量目前是占位符，烧录前必须替换为真实的 SSID 与密码：

```rust
const WIFI_SSID: &str = "x";
const WIFI_PASSWORD: &str = "x";
```

设备只会连接扫描结果中与 `WIFI_SSID` 完全一致的 AP。

### 2. MQTT Broker

在 `src/mqtt.rs` 中修改：

```rust
pub const MQTT_BROKER: &str = "192.168.3.15"; // Broker 的 IP（当前仅支持 IPv4 字面量）
pub const MQTT_PORT: u16 = 1883;
pub const MQTT_CLIENT_ID: &str = "AutoKeyboard";
pub const MQTT_TOPIC: &str = "keyboard/auto";
```

> 多台设备同时接入时请修改 `MQTT_CLIENT_ID`，避免客户端 ID 冲突导致互相踢下线。

### 3. 矩阵引脚

`src/main.rs` 中通过 `config_matrix_pins_esp!` 宏配置：

```rust
let (row_pins, col_pins) = config_matrix_pins_esp!(
    peripherals: peripherals,
    input:  [GPIO15, GPIO18, GPIO21, GPIO35, GPIO36, GPIO38],
    output: [GPIO10, ..., GPIO39]
);
```

行/列数量由 `src/keymap.rs` 的 `ROW`、`COL` 常量决定，改动引脚数量时需同步修改。

### 4. 键位映射

- **Vial 改键**：直接使用 Vial 客户端连接设备修改，配置会持久化到 Flash
- **默认键位**：`src/keymap.rs` 的 `get_default_keymap()`，仅在 Flash 中无有效配置时生效
- **Vial 布局描述**：项目根目录 `vial.json`，由 `build.rs` 在编译时压缩并生成 `VIAL_KEYBOARD_DEF`。注意 `build.rs` 中的 `keyboard_id` 需与固件保持一致，否则 Vial 无法识别设备

烧录后如需恢复出厂键位，可先擦除配置扇区（`espflash erase-flash`）再重新烧录。

## MQTT 远程按键协议

订阅主题：`keyboard/auto`

Payload 至少 10 字节，采用小端序：

| 偏移 | 长度 | 字段 | 说明 |
| --- | --- | --- | --- |
| 0 | 8 | `msg_id` | `u64`，消息 ID，小端序 |
| 8 | 1 | `key_code` | HID KeyCode，对应 `KEY_MAPPING` 中的 `key_code` |
| 9 | 1 | `pressed` | `0` = 松开，非 `0` = 按下 |

固件收到消息后，先按 HID 键值在 `src/keymap.rs` 的 `KEY_MAPPING` 表中查找对应的矩阵坐标 `(row, col)`，再通过 `rmk::event::publish_event_async` 注入按键事件。未在表中登记的键值会打印错误日志并被忽略。

`msg_id` 目前仅用于日志，未做去重处理。

### 发送示例（Python）

```python
import struct, paho.mqtt.client as mqtt

# HID KeyCode 'A' = 0x04，方向键上 = 0x52
payload = struct.pack("<QBB", msg_id=1, key_code=0x04, pressed=1)

client = mqtt.Client()
client.connect("192.168.3.15", 1883)
client.publish("keyboard/auto", payload, qos=0)
```

## 项目结构

```
.
├── build.rs          # 编译期生成 Vial 配置（压缩 vial.json + keyboard id）
├── vial.json         # Vial 布局描述（矩阵尺寸、物理布局）
├── Cargo.toml        # 依赖与编译 profile
└── src/
    ├── main.rs       # 入口：外设/USB/Flash 初始化、RMK 装配、WiFi 任务
    ├── keymap.rs     # 矩阵尺寸、HID 键值 ↔ 矩阵坐标映射表、默认键位
    ├── mqtt.rs       # MQTT 客户端任务：连接、订阅、解析报文并注入按键事件
    ├── vial.rs       # 引入 build.rs 生成的 Vial 配置
    └── macros.rs     # 矩阵引脚配置等宏
```

## 依赖说明

- [`rmk`](https://github.com/HaoboGu/rmk)：键盘固件框架（`esp32s3_ble`、`log`、`storage`、`vial` 特性）
- [`esp-hal`](https://github.com/esp-rs/esp-hal) / `esp-radio` / `esp-rtos`：ESP32-S3 硬件抽象与运行时
- `embassy-executor` / `embassy-net`：异步运行时与网络协议栈
- `rust-mqtt`：MQTT v5 客户端（`no_std`）

## 后续计划

- [ ] WiFi 凭证与 MQTT 配置改为运行时可配（Vial 或存储区内），不再硬编码
- [ ] MQTT 报文支持 `msg_id` 去重与 QoS 1
- [ ] 补全 Vial 布局描述，与实际 6×17 键盘匹配
- [ ] 增加 BLE 无线模式（相关代码已在 `main.rs` 中注释保留）
