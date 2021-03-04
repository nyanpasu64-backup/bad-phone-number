/// see https://github.com/hecrj/iced/blob/0.2/examples/progress_bar/src/main.rs
use iced::widget::slider;
use iced::{Column, Element, Row, Sandbox, Settings, Slider, Text};
use itertools::izip;
use mod_exp::mod_exp;

type Num = i32;
const NDIGITS: usize = 10;

#[derive(Default)]
struct State {
    digits: [Num; NDIGITS],
    sliders: [slider::State; NDIGITS],
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SetSlider { index: usize, value: Num },
}

impl Sandbox for State {
    type Message = Message;

    fn new() -> State {
        State::default()
    }

    fn title(&self) -> String {
        String::from("Polynomial phone number entry widget")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SetSlider { index, value } => {
                self.digits[index] = value;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // use random traits until one works
        use std::fmt::Write;

        // We use a column: a simple vertical layout
        let mut out = Column::new().push(
            Text::new({
                let mut s = String::new();
                for &d in &self.digits {
                    write!(&mut s, "{}", d).unwrap();
                }
                s
            })
            .size(50)
            .horizontal_alignment(iced::HorizontalAlignment::Center), // doesn't work
        );
        for (index, &value, _state) in izip!(0.., self.digits.iter(), self.sliders.iter_mut()) {
            out = out.push(
                Row::new()
                    .push(Text::new(format!("{}x^{}", value, index)))
                    .push(Slider::new(_state, 0..=9, value, move |value| {
                        Message::SetSlider { index, value }
                    })),
            );
        }
        out.into()
    }
}

fn main() -> iced::Result {
    State::run(Settings::default())
}
