use iced::widget::canvas;
use iced::{mouse, Color, Element, Point, Rectangle, Renderer, Size, Task, Theme};

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title(State::title)
        .centered()
        .theme(|_: &State| Theme::Dark)
        .run()
}

struct State {
    cube: HappyCube,
}

impl Default for State {
    fn default() -> Self {
        Self {
            cube: HappyCube { size: 300.0 },
        }
    }
}

enum Message {
    #[allow(dead_code)]
    None,
}

impl State {
    fn title(&self) -> String {
        "Happy Cube".to_string()
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        canvas(&self.cube)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

#[derive(Debug)]
struct HappyCube {
    size: f32,
}

impl<Message> canvas::Program<Message> for HappyCube {
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
        frame.fill(&body, Color::from_rgb(1.0, 0.5, 0.0));

        let eye_radius = size * 0.08;
        let eye_offset_x = size * 0.2;
        let eye_offset_y = size * 0.15;
        let left_eye_pos = Point::new(center.x - eye_offset_x, center.y - eye_offset_y);
        let right_eye_pos = Point::new(center.x + eye_offset_x, center.y - eye_offset_y);

        frame.fill(
            &canvas::Path::circle(left_eye_pos, eye_radius),
            Color::WHITE,
        );
        frame.fill(
            &canvas::Path::circle(right_eye_pos, eye_radius),
            Color::WHITE,
        );

        let pupil_radius = eye_radius * 0.5;
        frame.fill(
            &canvas::Path::circle(left_eye_pos, pupil_radius),
            Color::BLACK,
        );
        frame.fill(
            &canvas::Path::circle(right_eye_pos, pupil_radius),
            Color::BLACK,
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
                .with_color(Color::BLACK)
                .with_line_cap(canvas::LineCap::Round),
        );

        vec![frame.into_geometry()]
    }
}
