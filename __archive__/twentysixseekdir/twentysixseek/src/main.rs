use reqwest::Client;
use serde::Deserialize;
use std::fs;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct VideosListResponse {
    items: Vec<VideoItem>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
struct Snippet {
    title: String,
    #[serde(rename = "channelTitle", default)]
    channel_title: Option<String>,
    #[serde(rename = "publishedAt", default)]
    published_at: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ContentDetails {
    #[serde(default)]
    duration: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Statistics {
    #[serde(rename = "viewCount", default)]
    view_count: Option<String>,
    #[serde(rename = "likeCount", default)]
    like_count: Option<String>,
    #[serde(rename = "commentCount", default)]
    comment_count: Option<String>,
}

#[allow(dead_code)]
async fn fetch_videos_batch(
    client: &Client,
    api_key: &String,
    video_ids: Vec<String>,
) -> Result<VideosListResponse, reqwest::Error> {
    let ids_csv = video_ids.join(",");

    client
        .get("https://www.googleapis.com/youtube/v3/videos")
        .query(&[
            ("part", "snippet,contentDetails,statistics"),
            ("id", &ids_csv),
            ("key", api_key),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<VideosListResponse>()
        .await
}

#[tokio::main]
async fn main() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len == 4 && &args[1] == "youtube_1100" {
        let filename = &args[2];
        let api_key = &args[3];
        let mut video_ids: Vec<String> = Vec::new();
        let youtube_url_raw_list = read_lines(filename);
        let youtube_url_root = "https://www.youtube.com/watch?v=";
        for youtube_url_raw in youtube_url_raw_list {
            let youtube_url = &youtube_url_raw[youtube_url_root.len()..];
            //println!("youtube_url={}", youtube_url);
            video_ids.push(youtube_url.to_string());
        }

        let client = Client::new();
        let fetch = fetch_videos_batch(&client, &api_key, video_ids)
            .await
            .expect("ERR fetch_videos_batch");
        println!("fetch.items.len={}", fetch.items.len());
    }
    println!("<END>");
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
