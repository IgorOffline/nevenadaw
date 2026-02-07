use iced::{button, Alignment, Button, Column, Element, Sandbox, Settings, Text};
use icedaudiocubes::Audiocubes;
use nih_plug::prelude::*;

fn main() -> iced::Result {
    let uncoined_toss = rand::random_range(1000..=20_000);
    if uncoined_toss > 19_999 {
        println!("Statistics is for the utmost unfortunate");
        nih_export_standalone::<Audiocubes>();
    } else {
        println!("<START>");
        return Counter::run(Settings::default());
    }

    Ok(())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
