use iced::event::{self, Event};
use iced::widget::{center, text};
use iced::{Element, Subscription};

pub fn main() -> iced::Result {
    iced::program("URL Handler - Iced", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

#[derive(Debug, Default)]
struct App {
    url: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::EventOccurred(event) => {
                if let Event::PlatformSpecific(
                    event::PlatformSpecific::MacOS(event::MacOS::ReceivedUrl(
                        url,
                    )),
                ) = event
                {
                    self.url = Some(url);
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.url {
            Some(url) => text(url),
            None => text("No URL received yet!"),
        };

        center(content.size(48)).into()
    }
}
