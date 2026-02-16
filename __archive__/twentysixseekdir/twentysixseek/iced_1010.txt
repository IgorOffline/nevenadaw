use iced::widget::{button, column, text, Column};
use iced::Center;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct VideosListResponse {
    items: Vec<VideoItem>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct VideoItem {
    id: String,
    #[serde(default)]
    snippet: Option<Snippet>,
    #[serde(rename = "contentDetails", default)]
    content_details: Option<ContentDetails>,
    #[serde(default)]
    statistics: Option<Statistics>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Snippet {
    title: String,
    #[serde(rename = "channelTitle", default)]
    channel_title: Option<String>,
    #[serde(rename = "publishedAt", default)]
    published_at: Option<String>,
    #[serde(default)]
    thumbnails: Option<Thumbnails>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Thumbnails {
    #[serde(rename = "default", default)]
    default: Option<Thumbnail>,
    #[serde(default)]
    medium: Option<Thumbnail>,
    #[serde(default)]
    high: Option<Thumbnail>,
    #[serde(default)]
    standard: Option<Thumbnail>,
    #[serde(default)]
    maxres: Option<Thumbnail>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
struct Thumbnail {
    url: String,
    width: Option<u32>,
    height: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct ContentDetails {
    #[serde(default)]
    duration: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Statistics {
    #[serde(rename = "viewCount", default)]
    view_count: Option<String>,
    #[serde(rename = "likeCount", default)]
    like_count: Option<String>,
    #[serde(rename = "commentCount", default)]
    comment_count: Option<String>,
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
