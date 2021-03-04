/// see https://github.com/hecrj/iced/blob/0.2/examples/progress_bar/src/main.rs
use std::char;

use iced::widget::slider;
use iced::{Column, Element, Row, Sandbox, Settings, Slider, Text};
use itertools::izip;

type Num = u32;
const DECIMAL: u32 = 10;

struct State {
    format: String,
    digits: Vec<Num>,
    sliders: Vec<slider::State>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SetSlider { index: usize, value: Num },
}

impl State {
    fn from_string(format: String) -> State {
        let ndigits = format.chars().filter(|c| c.is_digit(DECIMAL)).count();
        return State {
            format,
            digits: vec![0; ndigits],
            sliders: vec![slider::State::default(); ndigits],
        };
    }
}

impl Sandbox for State {
    type Message = Message;

    fn new() -> State {
        State::from_string(String::from("1-111-111-1111"))
    }

    fn title(&self) -> String {
        String::from("Pascal's Pager")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SetSlider { index, value } => {
                self.digits[index] = value;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let mut out = Column::new().push(
            Text::new({
                let mut ys = vec![0; self.digits.len()];
                let mut pascal = vec![0; self.digits.len()];
                pascal[0] = 1;

                for &coeff in self.digits.iter() {
                    for (y, &p) in ys.iter_mut().zip(pascal.iter()) {
                        *y += coeff * p;
                        *y %= DECIMAL;
                    }
                    for i in (1..pascal.len()).rev() {
                        pascal[i] += pascal[i - 1];
                        pascal[i] %= DECIMAL;
                    }
                }

                let mut s = String::new();
                {
                    let mut ys = ys.iter();
                    for c in self.format.chars() {
                        if c.is_digit(DECIMAL) {
                            s.push(char::from_digit(*ys.next().unwrap(), DECIMAL).unwrap());
                        } else {
                            s.push(c)
                        }
                    }
                    assert!(ys.next().is_none());
                }
                s
            })
            .size(50)
            .horizontal_alignment(iced::HorizontalAlignment::Center), // doesn't work
        );
        for (index, &coeff, _state) in izip!(0.., self.digits.iter(), self.sliders.iter_mut()) {
            out = out.push(
                Row::new()
                    .push(Text::new(coeff.to_string()))
                    .push(Slider::new(
                        _state,
                        0..=(DECIMAL - 1),
                        coeff,
                        move |value| Message::SetSlider { index, value },
                    )),
            );
        }
        out.into()
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 300);
    State::run(settings)
}
