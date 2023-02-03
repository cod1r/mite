use iced::widget::{column, text_input};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    ChatBox::run(Settings::default())
}

struct ChatBox {
    value: String,
}

#[derive(Debug, Clone)]
enum Message {
    OnType(String),
}

impl Sandbox for ChatBox {
    type Message = Message;

    fn new() -> Self {
        Self { value: "".to_string() }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        let Message::OnType(s) = message;
        self.value = s;
    }

    fn view(&self) -> Element<Message> {
        column![
            text_input("", &self.value, Message::OnType),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
