use chrono::Datelike;
use num_format::{Locale, ToFormattedString};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::time::Duration;
use uuid::Uuid;

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

#[tokio::main]
async fn main() {
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

        let mut thumbnails: Vec<String> = Vec::new();
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
            thumbnails.push(medium_thumbnail_url.clone());
        }
        save_thumbnails(&thumbnails).await;
    }
    println!("<END>");
}

async fn save_thumbnails(thumbnails: &Vec<String>) {
    for thumbnail in thumbnails {
        let filename_timepart = prepare_filename_timepart();
        let thumbnail_filename = format!("thumbnail_{}.jpg", filename_timepart);
        download_image(thumbnail, &*thumbnail_filename)
            .await
            .expect("Cannot download image");
    }
}

async fn download_image(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(25))
        .build()?;

    println!("Target URL: {}", url);
    println!("Output file: {}", filename);

    let mut response = client.get(url).send().await?.error_for_status()?;

    if let Some(content_length) = response.content_length() {
        println!(
            "Content length: {} bytes",
            content_length.to_formatted_string(&Locale::en)
        );
    } else {
        println!("Content length: Unknown");
    }

    let mut file = File::create(filename)?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;

        print!(
            "\rDownloading: {} bytes",
            downloaded.to_formatted_string(&Locale::en)
        );
        use std::io::Write;
        std::io::stdout().flush()?;
    }

    println!("\nDownload completed successfully.");
    Ok(())
}

fn prepare_filename_timepart() -> String {
    let now = chrono::Utc::now();
    let year = now.year();
    let month = now.month().to_string();
    let day = now.day().to_string();
    let formatted_month = zero_leading_format(&month);
    let formatted_day = zero_leading_format(&day);
    let uuid_substring = &Uuid::new_v4().to_string()[..8].to_uppercase();
    let filename = format!(
        "{}-{}-{}_{}",
        year, formatted_month, formatted_day, uuid_substring
    );

    filename
}

fn zero_leading_format(input: &str) -> String {
    format!("{:0>2}", input)
}
