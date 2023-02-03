use iced::widget::{column, text_input};
use iced::{executor, keyboard};
use iced::{Alignment, Application, Command, Element, Settings, Subscription, Theme};
use iced_native::Event;

pub fn main() -> iced::Result {
    ChatBox::run(Settings::default())
}

struct ChatBox {
    value: String,
}

#[derive(Debug, Clone)]
enum Message {
    OnType(String),
    Ev(Event),
}

impl Application for ChatBox {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ChatBox, Command<Message>) {
        (
            ChatBox {
                value: "".to_string(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("mite")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OnType(s) => {
                self.value = s;
                Command::none()
            }
            Message::Ev(e) => {
                match e {
                    Event::Keyboard(keyboard::Event::CharacterReceived(k)) => {
                        if k as u8 == 13 {
                            println!("ENTER");
                        }
                    }
                    _ => {}
                }
                Command::none()
            }
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::Ev)
    }

    fn view(&self) -> Element<Message> {
        column![text_input("", &self.value, Message::OnType),]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}
