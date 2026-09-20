#![no_std]
#![no_main]

mod keymap;
#[macro_use]
mod macros;
mod mqtt;
mod vial;

use core::ptr::addr_of_mut;

use alloc::string::ToString;
// use bt_hci::controller::ExternalController;
use embassy_executor::Spawner;

use embassy_time::{Duration, Timer};
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::otg_fs::Usb;
use esp_hal::otg_fs::asynch::{Config, Driver};
use esp_hal::peripherals::TIMG1;
use esp_hal::rng::TrngSource;
use esp_hal::timer::timg::{TimerGroup, Wdt};
// use esp_radio::ble::controller::BleConnector;
use esp_radio::wifi::Ssid;
use esp_radio::wifi::scan::ScanConfig;
use esp_radio::wifi::sta::StationConfig;
use esp_storage::FlashStorage;
use log::{error, info};
// use rmk::ble::{BleTransport, build_ble_stack};
use rmk::config::{
    BehaviorConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig,
};
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::host::HostService;
use rmk::keyboard::Keyboard;
use rmk::matrix::Matrix;
use rmk::processor::builtin::wpm::WpmProcessor;
use rmk::storage::async_flash_wrapper;
use rmk::usb::UsbTransport;
use rmk::{KeymapData, embassy_time, initialize_keymap_and_storage, run_all};

use crate::keymap::*;
use crate::vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};

extern crate alloc;

::esp_bootloader_esp_idf::esp_app_desc!();

const WIFI_SSID: &str = "x";
const WIFI_PASSWORD: &str = "x";

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // Initialize the peripherals and bluetooth controller
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    esp_alloc::heap_allocator!(size: 72 * 1024);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let software_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);
    let _trng_source = TrngSource::new(peripherals.RNG, peripherals.ADC1);
    // let mut rng = esp_hal::rng::Trng::try_new().unwrap();

    let (wifi_controller, interfaces) = esp_radio::wifi::new(peripherals.WIFI, Default::default())
        .expect("Failed to initialize Wi-Fi controller");

    // let connector = BleConnector::new(peripherals.BT, Default::default()).unwrap();
    // let controller: ExternalController<_, 20> = ExternalController::new(connector);
    // let central_addr = [0x18, 0xe2, 0x21, 0x80, 0xc0, 0xc7];
    // let mut host_resources = HostResources::new();
    // let stack = build_ble_stack(controller, central_addr, &mut rng, &mut host_resources).await;

    // Initialize USB
    static mut EP_MEMORY: [u8; 1024] = [0; 1024];
    let usb = Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);
    // Create the driver, from the HAL.
    let config = Config::default();
    let usb_driver = Driver::new(usb, unsafe { &mut *addr_of_mut!(EP_MEMORY) }, config);

    // Initialize the flash
    let flash = FlashStorage::new(peripherals.FLASH);
    let flash = async_flash_wrapper(flash);

    // Initialize the IO pins
    let (row_pins, col_pins) = config_matrix_pins_esp!(
        peripherals: peripherals,
        input: [GPIO15, GPIO18, GPIO21, GPIO35, GPIO36, GPIO38],
        output: [GPIO10, GPIO9, GPIO8, GPIO7, GPIO6, GPIO5, GPIO4, GPIO2, GPIO11, GPIO12, GPIO13, GPIO14, GPIO16, GPIO17, GPIO34, GPIO37, GPIO39]
    );

    // RMK config
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[(0, 0), (1, 1)]);
    let storage_config = StorageConfig {
        start_addr: 0x3f0000,
        num_sectors: 16,
        ..Default::default()
    };
    let rmk_config = RmkConfig {
        device_config: DeviceConfig {
            vid: 0x4c4b,                           // 厂商ID
            pid: 0x4643,                           // 产品ID
            manufacturer: "RMK",                   // 制造商
            product_name: "AIKeyboard",            // 产品名称
            serial_number: "vial:f64c2b3c:000001", // 序列号
        },
        vial_config,
        storage_config,
        ..Default::default()
    };

    // Initialze keyboard stuffs
    // Initialize the storage and keymap
    let mut keymap_data = KeymapData::new(keymap::get_default_keymap());
    let mut behavior_config = BehaviorConfig::default();
    let per_key_config = PositionalConfig::default();
    let (keymap, mut storage) = initialize_keymap_and_storage(
        &mut keymap_data,
        flash,
        &storage_config,
        &mut behavior_config,
        &per_key_config,
    )
    .await;

    // Initialize the matrix and keyboard
    let debouncer = DefaultDebouncer::new();
    let mut matrix = Matrix::<_, _, _, ROW, COL, true>::new(row_pins, col_pins, debouncer);
    // let mut matrix = rmk::matrix::TestMatrix::<ROW, COL>::new();
    let mut keyboard = Keyboard::new(&keymap); // Initialize the light controller
    let host_ctx = rmk::host::KeyboardContext::new(&keymap);
    let mut host_service = HostService::new(&host_ctx, &rmk_config);

    let mut usb_transport = UsbTransport::new(usb_driver, rmk_config.device_config);
    // let mut ble_transport = BleTransport::new(&stack, rmk_config).await;
    let mut wpm_processor = WpmProcessor::new();

    // Create network stack with DHCP
    let net_stack_resources = mk_static!(
        embassy_net::StackResources<3>,
        embassy_net::StackResources::new()
    );
    let (net_stack, net_runner) = embassy_net::new(
        interfaces.station,
        embassy_net::Config::dhcpv4(embassy_net::DhcpConfig::default()),
        net_stack_resources,
        0x080, // random seed
    );
    let net_stack = mk_static!(embassy_net::Stack<'static>, net_stack);

    // Spawn network task
    spawner.spawn(net_task(net_runner).expect("Failed to spawn network task"));

    // Static reference for WiFi controller
    let wifi_controller = mk_static!(esp_radio::wifi::WifiController<'static>, wifi_controller);

    // Spawn WiFi connection task with network stack
    spawner
        .spawn(wifi_task(wifi_controller, net_stack, spawner).expect("Failed to spawn WiFi task"));

    // TODO: Spawn some tasks

    run_all!(
        matrix,
        storage,
        usb_transport,
        // ble_transport,
        wpm_processor,
        keyboard,
        host_service
    )
    .await;
}

// Network task - runs the embassy-net runner
#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, esp_radio::wifi::Interface<'static>>) {
    runner.run().await;
}

// WiFi connection task
#[embassy_executor::task]
async fn wifi_task(
    wifi_controller: &'static mut esp_radio::wifi::WifiController<'static>,
    net_stack: &'static embassy_net::Stack<'static>,
    spawner: Spawner,
) {
    loop {
        if !wifi_controller.is_connected() {
            find_and_connect_wifi(wifi_controller, net_stack, spawner).await;
        }
        Timer::after(Duration::from_secs(5)).await;
    }
}

async fn find_and_connect_wifi(
    wifi_controller: &mut esp_radio::wifi::WifiController<'_>,
    net_stack: &'static embassy_net::Stack<'static>,
    spawner: Spawner,
) {
    // Scan for WiFi networks
    let scan_config = ScanConfig::default().with_ssid(WIFI_SSID).with_max(20);
    match wifi_controller.scan_async(&scan_config).await {
        Ok(aps) => {
            let mut found = false;

            for ap in aps {
                let ssid = ap.ssid.as_str();
                if ssid == WIFI_SSID {
                    found = true;
                    break;
                }
            }

            if !found {
                return;
            }

            // Configure WiFi connection with SSID and password
            let station_config = StationConfig::default()
                .with_ssid(Ssid::from(WIFI_SSID))
                .with_password(WIFI_PASSWORD.to_string());

            if let Err(e) =
                wifi_controller.set_config(&esp_radio::wifi::Config::Station(station_config))
            {
                error!("Failed to set WiFi config: {:?}", e);
                return;
            }

            // Connect to WiFi
            match wifi_controller.connect_async().await {
                Ok(info) => {
                    info!("Connected to WiFi '{}': {:?}", WIFI_SSID, info);

                    // Wait for DHCP to assign IP address
                    info!("Waiting for DHCP configuration...");
                    net_stack.wait_config_up().await;

                    // Get and print IP address
                    if let Some(config) = net_stack.config_v4() {
                        info!("DHCP configured! IP address: {}", config.address.address());
                        if let Some(gateway) = config.gateway {
                            info!("Gateway: {}", gateway);
                        }
                        for dns in config.dns_servers {
                            info!("DNS server: {}", dns);
                        }
                    }

                    // mqtt 连接
                    spawner.spawn(
                        crate::mqtt::mqtt_task(net_stack).expect("Failed to spawn MQTT task"),
                    );

                    // 触发按键事件
                    // rmk::event::publish_event_async(rmk::event::KeyboardEvent::key(0, 0, true))
                    //     .await;
                    // info!("Key 0, 0 pressed");
                }
                Err(e) => {
                    error!("Failed to connect to WiFi '{}': {:?}", WIFI_SSID, e);
                }
            }
        }
        Err(e) => {
            error!("Failed to scan WiFi: {:?}", e);
        }
    }
}

// Watchdog that gets fed every 500 ms
#[embassy_executor::task]
async fn watchdog_task(watchdog: &'static mut Wdt<TIMG1<'static>>) {
    loop {
        watchdog.feed();
        Timer::after(Duration::from_millis(500)).await;
    }
}
