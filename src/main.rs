use iced::widget::{column, text, text_input, Container, Scrollable};
use iced::{executor, keyboard};
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use iced_native::Event;

pub fn main() -> iced::Result {
    ChatBox::run(Settings::default())
}

struct ChatBox {
    value: String,
    hist: Vec<String>,
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
                hist: Vec::new(),
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
                        if k as u8 == 13 && self.value.len() > 0 {
                            self.hist.push(self.value.clone());
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
        let mut hist_str = String::new();
        for s in &self.hist {
            hist_str += s;
            hist_str += "\n";
        }
        Container::new(
            column![
                Scrollable::new(text(hist_str)).height(Length::Fill),
                column![text_input("", &self.value, Message::OnType)],
            ]
            .padding(20)
            .align_items(Alignment::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
