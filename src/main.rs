/// see https://github.com/hecrj/iced/blob/0.2/examples/progress_bar/src/main.rs
use iced::widget::slider;
use iced::{Column, Element, Row, Sandbox, Settings, Slider, Text};
use itertools::izip;

type Num = i32;
const NDIGITS: usize = 10;
const DECIMAL: i32 = 10;

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
        // use random traits until one works
        use std::fmt::Write;

        // We use a column: a simple vertical layout
        let mut out = Column::new().push(
            Text::new({
                let mut ys = [0; NDIGITS];
                let mut pascal = [0; NDIGITS];
                pascal[0] = 1;

                for &coeff in self.digits.iter() {
                    for (y, &p) in ys.iter_mut().zip(pascal.iter()) {
                        *y += coeff * p;
                    }
                    for i in (1..pascal.len()).rev() {
                        pascal[i] += pascal[i - 1];
                        pascal[i] %= DECIMAL;
                    }
                }

                let mut s = String::new();
                for &y in &ys {
                    write!(&mut s, "{}", y % DECIMAL).unwrap();
                }
                s
            })
            .size(50)
            .horizontal_alignment(iced::HorizontalAlignment::Center), // doesn't work
        );
        for (index, &coeff, _state) in izip!(0.., self.digits.iter(), self.sliders.iter_mut()) {
            out = out.push(
                Row::new()
                    .push(Text::new(index.to_string()))
                    .push(Slider::new(
                        _state,
                        0..=(DECIMAL - 1),
                        coeff,
                        move |value| Message::SetSlider { index, value },
                    ))
            );
        }
        out.into()
    }
}

fn main() -> iced::Result {
    State::run(Settings::default())
}
