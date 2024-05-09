use iced::widget::{center, column, container, slider, text, vertical_slider};
use iced::Element;

pub fn main() -> iced::Result {
    iced::run("Slider - Iced", Slider::update, Slider::view)
}

#[derive(Debug, Clone)]
pub enum Message {
    SliderChanged(u8),
}

pub struct Slider {
    value: u8,
    default: u8,
    step: u8,
    shift_step: u8,
}

impl Slider {
    fn new() -> Self {
        Slider {
            value: 50,
            default: 50,
            step: 5,
            shift_step: 1,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(value) => {
                self.value = value;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let h_slider = container(
            slider(0..=100, self.value, Message::SliderChanged)
                .default(self.default)
                .step(self.step)
                .shift_step(self.shift_step),
        )
        .width(250);

        let v_slider = container(
            vertical_slider(0..=100, self.value, Message::SliderChanged)
                .default(self.default)
                .step(self.step)
                .shift_step(self.shift_step),
        )
        .height(200);

        let text = text(self.value);

        center(
            column![
                container(v_slider).center_x(),
                container(h_slider).center_x(),
                container(text).center_x()
            ]
            .spacing(25),
        )
        .into()
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}
