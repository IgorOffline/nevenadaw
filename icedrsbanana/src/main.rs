use iced::widget::canvas;
use iced::{color, mouse, Element, Point, Rectangle, Renderer, Size, Task, Theme};
use std::time::Duration;

fn main() -> iced::Result {
    iced::application(State::init, State::update, State::view)
        .title(State::title)
        .centered()
        .theme(|_: &State| Theme::Dark)
        .run()
}

struct State {
    cube: BananaCube,
}

impl Default for State {
    fn default() -> Self {
        Self {
            cube: BananaCube {
                size: 300.0,
                counter: 0,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

impl State {
    fn init() -> (Self, Task<Message>) {
        (Self::default(), Task::done(Message::Tick))
    }

    fn title(&self) -> String {
        "Banana Cube".to_string()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.cube.counter += 1;

                Task::perform(tokio::time::sleep(Duration::from_millis(200)), |_| {
                    Message::Tick
                })
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        canvas(&self.cube)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

#[derive(Debug)]
struct BananaCube {
    size: f32,
    counter: u32,
}

impl<Message> canvas::Program<Message> for BananaCube {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let center = frame.center();
        let size = self.size.min(bounds.width).min(bounds.height) * 0.8;

        let body = canvas::Path::rectangle(
            Point::new(center.x - size / 2.0, center.y - size / 2.0),
            Size::new(size, size),
        );
        let color_primary = color!(0xffa000);
        frame.fill(&body, color_primary);

        let eye_radius = size * 0.08;
        let eye_offset_x = size * 0.2;
        let eye_offset_y = size * 0.15;
        let left_eye_pos = Point::new(center.x - eye_offset_x, center.y - eye_offset_y);
        let right_eye_pos = Point::new(center.x + eye_offset_x, center.y - eye_offset_y);

        let color_secondary_text = color!(0x757575);
        frame.fill(
            &canvas::Path::circle(left_eye_pos, eye_radius),
            color_secondary_text,
        );
        frame.fill(
            &canvas::Path::circle(right_eye_pos, eye_radius),
            color_secondary_text,
        );

        let color_primary_text = color!(0x212121);
        let pupil_radius = eye_radius * 0.5 + self.counter as f32 * 0.5;
        frame.fill(
            &canvas::Path::circle(left_eye_pos, pupil_radius),
            color_primary_text,
        );
        frame.fill(
            &canvas::Path::circle(right_eye_pos, pupil_radius),
            color_primary_text,
        );

        let smile = canvas::Path::new(|b| {
            b.move_to(Point::new(center.x - size * 0.25, center.y + size * 0.1));
            b.quadratic_curve_to(
                Point::new(center.x, center.y + size * 0.35),
                Point::new(center.x + size * 0.25, center.y + size * 0.1),
            );
        });
        frame.stroke(
            &smile,
            canvas::Stroke::default()
                .with_width(size * 0.04)
                .with_color(color_primary_text)
                .with_line_cap(canvas::LineCap::Round),
        );

        vec![frame.into_geometry()]
    }
}
