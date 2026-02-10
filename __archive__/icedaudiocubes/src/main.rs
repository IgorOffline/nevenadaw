use iced::canvas::{self, Canvas, Geometry, Program};
use iced::{Color, Element, Rectangle, Sandbox, Settings};
use icedaudiocubes::Audiocubes;
use nih_plug::prelude::*;

fn main() -> iced::Result {
    let uncoined_toss = rand::random_range(1000..=20_000);
    if uncoined_toss < 19_999 {
        println!("Statistics is for the utmost unfortunate");
        nih_export_standalone::<Audiocubes>();
    } else {
        println!("<START>");
        return HelloWorld::run(Settings::default());
    }

    Ok(())
}

struct HelloWorld;

impl Sandbox for HelloWorld {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Hello World Canvas - Iced")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> Element<'_, Self::Message> {
        Canvas::new(HelloCanvas)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

struct HelloCanvas;

impl Program<()> for HelloCanvas {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<Geometry> {
        let mut frame = canvas::Frame::new(bounds.size());

        frame.fill_text(canvas::Text {
            content: String::from("Hello, World!"),
            position: bounds.center(),
            color: Color::BLACK,
            size: 40.0.into(),
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            ..Default::default()
        });

        vec![frame.into_geometry()]
    }
}
