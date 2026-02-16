use serde::{Deserialize, Serialize};
use std::fs::File;

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

fn main() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    if args_len == 4 && &args[1] == "youtube_1101" {
        let mut videos: Vec<VideoItem> = Vec::new();
        let _api_key = &args[2];
        let comma_separated_json_files = &args[3];

        for filename in comma_separated_json_files
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            let file = File::open(filename).expect("Cannot open file");

            let resp: VideosListResponse =
                serde_json::from_reader(file).expect("Cannot parse file as VideosListResponse");

            videos.extend(resp.items);
        }

        println!("videos.len()={}", videos.len());

        videos.sort_by(|a, b| {
            let b_views: u64 = b
                .statistics
                .as_ref()
                .unwrap()
                .view_count
                .as_ref()
                .unwrap()
                .parse()
                .unwrap();
            let a_views: u64 = a
                .statistics
                .as_ref()
                .unwrap()
                .view_count
                .as_ref()
                .unwrap()
                .parse()
                .unwrap();
            b_views.cmp(&a_views)
        });

        for video in videos.iter().take(5) {
            let medium_thumbnail_url = video
                .snippet
                .as_ref()
                .unwrap()
                .thumbnails
                .as_ref()
                .unwrap()
                .medium
                .as_ref()
                .unwrap()
                .url
                .clone();
            println!("medium_thumbnail_url={}", medium_thumbnail_url);
        }
    }
    println!("<END>");
}
