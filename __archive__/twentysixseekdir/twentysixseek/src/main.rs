use iced::time::Instant;
use iced::widget::{center, image};
use iced::{Element, Fill, Subscription, Task, Theme};

pub fn main() -> iced::Result {
    iced::application::timed(Hello::new, Hello::update, Hello::subscription, Hello::view)
        .window(iced::window::Settings {
            size: [512.0, 512.0].into(),
            ..Default::default()
        })
        .theme(Hello::theme)
        .run()
}

struct Hello {
    now: Instant,
    picture: image::Handle,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Hello {
    fn new() -> (Self, Task<Message>) {
        let picture = image::Handle::from_path("assets/alice1a_alice1a.png");

        (
            Self {
                now: Instant::now(),
                picture,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message, now: Instant) -> Task<Message> {
        self.now = now;

        match message {
            Message::Tick => Task::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        center(image(&self.picture).width(256).height(256))
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(iced::time::Duration::from_millis(10)).map(|_| Message::Tick)
    }
}
