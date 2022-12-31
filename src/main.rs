#![allow(unused_imports)]
use iced::{
    alignment,
    widget::{button, column, row, text, text_input, Column, Row},
    Alignment, Element, Length, Sandbox,
};

fn main() -> iced::Result {
    State::run(iced::Settings::default())
}

#[derive(Default)]
struct State {
    row_one: [String; 4],
    row_two: [Option<i64>; 4],
    row_three: [Option<i64>; 4],
    row_four: [Option<i64>; 4],
}

#[derive(Debug, Clone)]
enum Message {
    SetRowOneCell { idx: usize, value: String },
}

impl Sandbox for State {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Birthday Magic Square")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SetRowOneCell { idx, value } => {
                if value.parse::<u32>().is_ok() || value.is_empty() {
                    self.row_one[idx] = value;

                    if self.row_one.iter().all(|s| !s.is_empty()) {
                        let a = self.row_one[0].parse::<i64>().unwrap() - 1;
                        let b = self.row_one[1].parse::<i64>().unwrap() - 2;
                        let c = self.row_one[2].parse::<i64>().unwrap() - 3;
                        let d = self.row_one[3].parse::<i64>().unwrap() - 4;

                        self.row_two = [Some(c + 4), Some(d + 3), Some(a + 2), Some(b + 1)];
                        self.row_three = [Some(d + 2), Some(c + 1), Some(b + 4), Some(a + 3)];
                        self.row_four = [Some(b + 3), Some(a + 4), Some(d + 1), Some(c + 2)];
                    } else {
                        self.row_two = Default::default();
                        self.row_three = Default::default();
                        self.row_four = Default::default();
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            Row::with_children(
                self.row_one
                    .iter()
                    .enumerate()
                    .map(|(idx, cell)| text_input(cell, cell, move |value| {
                        Message::SetRowOneCell { idx, value }
                    })
                    .into())
                    .collect()
            ),
            Row::with_children(
                self.row_two
                    .iter()
                    .map(|cell| text(cell.map(|i| i.to_string()).unwrap_or_default())
                        // .horizontal_alignment(alignment::Horizontal::Right)
                        .width(Length::Fill)
                        .into())
                    .collect()
            ),
            Row::with_children(
                self.row_three
                    .iter()
                    .map(|cell| text(cell.map(|i| i.to_string()).unwrap_or_default())
                        // .horizontal_alignment(alignment::Horizontal::Right)
                        .width(Length::Fill)
                        .into())
                    .collect()
            ),
            Row::with_children(
                self.row_four
                    .iter()
                    .map(|cell| text(cell.map(|i| i.to_string()).unwrap_or_default())
                        // .horizontal_alignment(alignment::Horizontal::Right)
                        .width(Length::Fill)
                        .into())
                    .collect()
            )
        ]
        .into()
    }
}
