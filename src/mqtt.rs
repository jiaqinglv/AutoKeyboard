use crate::embassy_time::{Duration, Timer};
use crate::keymap::key_code_to_pos;
use core::ptr::addr_of_mut;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Ipv4Address, Stack};
use log::{error, info};
use rust_mqtt::{
    buffer::BumpBuffer,
    client::Client,
    client::event::Event,
    client::options::{ConnectOptions, SubscriptionOptions},
    types::{MqttString, TopicName},
};

pub const MQTT_BROKER: &str = "192.168.3.15";
pub const MQTT_PORT: u16 = 1883;
pub const MQTT_CLIENT_ID: &str = "AutoKeyboard";
pub const MQTT_TOPIC: &str = "keyboard/auto";

const MAX_SUBSCRIBES: usize = 10;
const RECEIVE_MAXIMUM: usize = 10;
const SEND_MAXIMUM: usize = 10;
const MAX_SUBSCRIPTION_IDENTIFIERS: usize = 10;

static mut MQTT_BUFFER: [u8; 4096] = [0; 4096];
static mut TCP_RX_BUFFER: [u8; 4096] = [0; 4096];
static mut TCP_TX_BUFFER: [u8; 4096] = [0; 4096];

/// 自定义按键消息结构体
pub struct AutoKeyboardMessage {
    pub msg_id: u64,
    pub key_code: u8,
    pub pressed: bool,
}

#[embassy_executor::task]
pub async fn mqtt_task(net_stack: &'static Stack<'static>) {
    loop {
        info!(
            "Attempting MQTT connection to {}:{}",
            MQTT_BROKER, MQTT_PORT
        );

        let addr = match MQTT_BROKER.parse::<Ipv4Address>() {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to parse MQTT broker address: {:?}", e);
                Timer::after(Duration::from_secs(10)).await;
                continue;
            }
        };

        let mut socket = TcpSocket::new(
            *net_stack,
            unsafe { &mut *addr_of_mut!(TCP_RX_BUFFER) },
            unsafe { &mut *addr_of_mut!(TCP_TX_BUFFER) },
        );

        let stream = match socket.connect((addr, MQTT_PORT)).await {
            Ok(()) => socket,
            Err(e) => {
                error!("Failed to connect to MQTT broker: {:?}", e);
                Timer::after(Duration::from_secs(10)).await;
                continue;
            }
        };

        let mut buffer = unsafe { BumpBuffer::new(&mut *addr_of_mut!(MQTT_BUFFER)) };
        let mut client = Client::<
            _,
            _,
            MAX_SUBSCRIBES,
            RECEIVE_MAXIMUM,
            SEND_MAXIMUM,
            MAX_SUBSCRIPTION_IDENTIFIERS,
        >::new(&mut buffer);

        // TODO: 客户端ID-多设备时要注意
        let client_id = match MqttString::from_str(MQTT_CLIENT_ID) {
            Ok(id) => id,
            Err(e) => {
                error!("Failed to create client ID: {:?}", e);
                Timer::after(Duration::from_secs(10)).await;
                continue;
            }
        };

        let connect_options = ConnectOptions::new().clean_start();

        // 进行连接
        match client
            .connect(stream, &connect_options, Some(client_id))
            .await
        {
            Ok(_) => {
                info!("MQTT connected successfully");
            }
            Err(e) => {
                error!("MQTT connect failed: {:?}", e);
                Timer::after(Duration::from_secs(10)).await;
                continue;
            }
        }

        let topic_str = match MqttString::from_str(MQTT_TOPIC) {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to create topic string: {:?}", e);
                Timer::after(Duration::from_secs(10)).await;
                continue;
            }
        };

        let topic = match TopicName::new(topic_str) {
            Some(t) => t,
            None => {
                error!("Failed to create topic name");
                Timer::after(Duration::from_secs(10)).await;
                continue;
            }
        };

        if let Err(e) = client
            .subscribe(topic.as_borrowed().into(), SubscriptionOptions::new())
            .await
        {
            error!("Failed to subscribe: {:?}", e);
            Timer::after(Duration::from_secs(10)).await;
            continue;
        }

        info!("Subscribed to topic: {}", MQTT_TOPIC);

        loop {
            match client.poll().await {
                Ok(event) => match event {
                    Event::Publish(publish) => {
                        info!("Received MQTT message: topic={:?}", publish.topic);
                        if publish.topic == topic {
                            let payload = publish.message.as_bytes();
                            if payload.len() >= 10 {
                                let msg_id = u64::from_le_bytes(
                                    payload[0..8].try_into().unwrap_or_default(),
                                );
                                let key_code = payload[8];
                                let pressed = payload[9] != 0;

                                let msg = AutoKeyboardMessage {
                                    msg_id,
                                    key_code,
                                    pressed,
                                };
                                let pos = key_code_to_pos(msg.key_code);
                                match pos {
                                    Some(pos) => {
                                        info!("Key position: {:?}", pos);
                                        let row = pos.0;
                                        let col = pos.1;

                                        // 触发按键事件
                                        rmk::event::publish_event_async(
                                            rmk::event::KeyboardEvent::key(row, col, msg.pressed),
                                        )
                                        .await;
                                    }
                                    None => {
                                        error!("Invalid key code: {:?}", msg.key_code);
                                    }
                                };

                                info!(
                                    "Received AutoKeyboardMessage: msg_id={}, key_code={}, pressed={}",
                                    msg.msg_id, msg.key_code, msg.pressed
                                );
                            } else {
                                error!(
                                    "Invalid message length: expected at least 10 bytes, got {}",
                                    payload.len()
                                );
                            }
                        }
                    }
                    Event::PublishComplete(_) => {
                        info!("Publish complete");
                    }
                    Event::Suback(_) => {
                        info!("SubAck received");
                    }
                    other => {
                        info!("Other MQTT event: {:?}", other);
                    }
                },
                Err(e) => {
                    error!("MQTT poll error: {:?}", e);
                    break;
                }
            }
        }

        Timer::after(Duration::from_secs(10)).await;
    }
}
