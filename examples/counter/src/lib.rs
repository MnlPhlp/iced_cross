use iced::{
    theme::{Base, Mode},
    widget::{button, column, container, text, text_input},
    Alignment, Color, Element, Length, Renderer, Task, Theme,
};
use iced_cross::IcedApp;

iced_cross::lib!(Counter);

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    StartScan,
    UpdateDevices(Vec<String>),
    ScanStarted,
    ScanFinished,
    UpdateText(String),
}

#[derive(Debug, Default)]
pub struct Counter {
    value: i32,
    devices: Vec<String>,
    scan_running: bool,
    text_value: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn start_scan() -> Task<Message> {
    use futures::SinkExt;

    let (mut stream_tx, stream_rx) = futures::channel::mpsc::unbounded();
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    Task::batch([
        Task::future(async move {
            // start scanning
            let handler = tauri_plugin_blec::get_handler().expect("Failed to get BLE handler");
            handler
                .discover(
                    Some(tx),
                    1000,
                    tauri_plugin_blec::models::ScanFilter::None,
                    false,
                )
                .await
                .expect("Failed to start BLE scan");
            Message::ScanStarted
        }),
        Task::future(async move {
            while let Some(devices) = rx.recv().await {
                println!("Received BLE devices: {devices:?}");
                let device_names = devices
                    .into_iter()
                    .filter_map(|d| {
                        if d.name.is_empty() {
                            None
                        } else {
                            Some(d.name)
                        }
                    })
                    .collect::<Vec<_>>();
                stream_tx
                    .send(Message::UpdateDevices(device_names))
                    .await
                    .expect("Failed to send devices to stream");
            }
            stream_tx
                .send(Message::ScanFinished)
                .await
                .expect("Failed to send scan finished");
            Message::ScanFinished
        }),
        Task::stream(stream_rx),
    ])
}

#[cfg(target_arch = "wasm32")]
fn start_scan() -> Task<Message> {
    // BLE scanning is not supported in WASM, so we just do a dummy async task that simulates a scan delay
    Task::done(Message::ScanStarted)
        .chain(Task::done(Message::UpdateDevices(vec![
            "Scanning not supported in WASM".to_string(),
            "Dummy 1".to_string(),
            "Dummy 2".to_string(),
        ])))
        .chain(Task::future(async {
            // Simulate a scan delay
            gloo_timers::future::TimeoutFuture::new(2000).await;
            Message::ScanFinished
        }))
}

impl IcedApp for Counter {
    type Message = Message;

    #[cfg(not(target_arch = "wasm32"))]
    fn init_plugins(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
        builder.plugin(tauri_plugin_blec::init())
    }

    fn new() -> (Self, Task<Self::Message>) {
        (Self::default(), Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::StartScan => {
                return start_scan();
            }
            Message::ScanStarted => {
                self.scan_running = true;
            }
            Message::ScanFinished => {
                self.scan_running = false;
            }
            Message::UpdateDevices(devices) => {
                self.devices = devices;
            }
            Message::UpdateText(text) => {
                self.text_value = text;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Theme, Renderer> {
        let counter = column![
            button("Increment").on_press(Message::Increment),
            text(format!("Value: {}", self.value)).size(30),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(20)
        .spacing(10)
        .align_x(Alignment::Center);

        let text_input = text_input("Type something...", &self.text_value)
            .padding(10)
            .size(20)
            .on_input(|value| Message::UpdateText(value));

        let ble_scan = {
            let start_stop_button = if self.scan_running {
                button("Scanning...").style(|_theme, _status| button::Style {
                    background: Some(Color::from_rgb(0.8, 0.2, 0.2).into()),
                    ..Default::default()
                })
            } else {
                button("Start BLE Scan").on_press(Message::StartScan)
            };
            column![
                start_stop_button,
                text("Devices:").size(30),
                column(self.devices.iter().map(|d| text(d).into())).spacing(5)
            ]
            .spacing(10)
            .align_x(Alignment::Center)
        };

        let content = column![counter, text_input, ble_scan]
            .padding(20)
            .spacing(40)
            .align_x(Alignment::Center);

        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| container::Style {
                background: if matches!(theme.mode(), Mode::Dark) {
                    Some(Color::from_rgb(0.1, 0.1, 0.15).into())
                } else {
                    Some(Color::from_rgb(0.9, 0.9, 0.95).into())
                },
                ..Default::default()
            })
            .into()
    }
}
