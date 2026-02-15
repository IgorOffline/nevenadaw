use chrono::Datelike;
use num_format::{Locale, ToFormattedString};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use uuid::Uuid;
use walkdir::WalkDir;

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

        videos.iter().take(5555).for_each(|video| {
            let view_count = &&video
                .statistics
                .as_ref()
                .unwrap()
                .view_count
                .as_ref()
                .unwrap();
            let snippet = &video.snippet.as_ref().unwrap();
            let title = &&snippet.title;
            let medium_thumbnail_url = &snippet
                .thumbnails
                .as_ref()
                .unwrap()
                .medium
                .as_ref()
                .unwrap()
                .url;
            println!(
                "title={} view_count={}, medium_thumbnail_url={}",
                title, view_count, medium_thumbnail_url
            );
        })
    }
    println!("<END>");
}

#[allow(dead_code)]
fn main_videos() {
    let folder = r"C:\Users\igor\dev\youtube_january";
    let mut videos: Vec<VideoItem> = Vec::new();
    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
        })
    {
        let file = File::open(entry.path()).expect("Cannot open file");
        let videos_json =
            serde_json::from_reader::<_, VideosListResponse>(file).expect("Cannot parse file");
        for video_item in videos_json.items {
            videos.push(video_item);
        }
    }
    println!("videos.len={}", videos.len());
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
    let mut video_info_list: Vec<String> = Vec::new();
    videos.iter().take(9999).for_each(|video| {
        let title = &video.snippet.as_ref().unwrap().title;
        let view_count: u64 = video
            .statistics
            .as_ref()
            .unwrap()
            .view_count
            .as_ref()
            .unwrap()
            .parse()
            .unwrap();
        let formatted_view_count = view_count.to_formatted_string(&Locale::hr);
        let video_info = format!("[{} {}]", title, formatted_view_count);
        video_info_list.push(video_info);
    });
    let video_info_list_filename = prepare_filename(true);
    fs::write(video_info_list_filename, video_info_list.join("\n")).expect("Unable to write file");
}

#[allow(dead_code)]
fn main_chunk_per_50() {
    println!("<START>");
    let january = r"C:\D\notes\january";
    let youtube_url_root = "https://www.youtube.com/watch?v=";
    let mut video_ids: Vec<String> = Vec::new();
    fs::read_dir(january)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "txt"))
        .for_each(|file_content| {
            for youtube_url_raw in fs::read_to_string(file_content).unwrap().lines() {
                if stereotypical_youtube_video_url_valid(youtube_url_raw) {
                    let youtube_url = &youtube_url_raw[youtube_url_root.len()..];
                    video_ids.push(youtube_url.to_string());
                }
            }
        });
    println!("video_ids.len={}", video_ids.len());
    video_ids.chunks(50).for_each(|chunk| {
        println!("chunk.len={}", chunk.len());
        let filename = prepare_filename(true);
        fs::write(filename, chunk.join("\n")).expect("Unable to write file");
    });
    println!("<END>");
}

#[allow(dead_code)]
fn stereotypical_youtube_video_url_valid(url: &str) -> bool {
    let pattern = r"^https://www\.youtube\.com/watch\?v=[a-zA-Z0-9_-]{11}$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(url)
}

#[allow(dead_code)]
#[tokio::main]
async fn main_youtube_1100() {
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

fn prepare_filename(txt: bool) -> String {
    let now = chrono::Utc::now();
    let year = now.year();
    let month = now.month().to_string();
    let day = now.day().to_string();
    let formatted_month = zero_leading_format(&month);
    let formatted_day = zero_leading_format(&day);
    let uuid_substring = &Uuid::new_v4().to_string()[..8].to_uppercase();
    let extension = if txt { "txt" } else { "json" };
    let filename = format!(
        "{}-{}-{}_{}.{}",
        year, formatted_month, formatted_day, uuid_substring, extension
    );

    filename
}

fn zero_leading_format(input: &str) -> String {
    format!("{:0>2}", input)
}
