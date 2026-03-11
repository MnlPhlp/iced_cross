use iced::{
    widget::{button, column, container, text},
    Alignment, Color, Element, Length, Renderer, Task, Theme,
};
use iced_cross::IcedApp;

iced_cross::lib!(Counter);

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
}

#[derive(Debug, Default)]
pub struct Counter {
    value: i32,
}

impl IcedApp for Counter {
    type Message = Message;

    fn new() -> (Self, Task<Self::Message>) {
        (Self::default(), Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Theme, Renderer> {
        let content = column![
            button("Increment").on_press(Message::Increment),
            text(format!("Value: {}", self.value)).size(30),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(20)
        .spacing(10)
        .align_x(Alignment::Center);

        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color::from_rgb(0.1, 0.1, 0.15).into()),
                ..Default::default()
            })
            .into()
    }
}
