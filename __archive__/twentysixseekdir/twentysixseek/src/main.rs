use chrono::Datelike;
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

#[derive(Clone, Deserialize)]
struct Youtube {
    winter_2021: String,
    example_url: String,
    general_json_example_url: String,
    stereotypical_youtube_video_url_root: String,
    youtube_api_v3_base_url: String,
    youtube_api_v3_key_parts: String,
}

#[derive(Clone, Deserialize)]
struct YoutubeConfig {
    youtube: Youtube,
}

#[derive(Clone, Deserialize)]
struct Steam {
    player_id: String,
    key: String,
    url_raw: String,
}

#[derive(Clone, Deserialize)]
struct SteamConfig {
    steam: Steam,
}

#[tokio::main]
async fn main() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    args.clone()
        .into_iter()
        .for_each(|arg| println!("arg={}", arg));
    if args_len > 1 {
        if &args[1] == "external_games" {
            let client_id = &args[2];
            let token = &args[3];
            let query = r#"fields game, uid; where uid = "730";"#;
            let client = Client::new();
            let response = client
                .post("https://api.igdb.com/v4/games")
                .header("Client-ID", client_id)
                .header("Authorization", format!("Bearer {}", token))
                .body(query)
                .send()
                .await
                .expect("Failed to send request");

            let text = response.text().await.expect("Failed to read response body");
            println!("--- --- ---");
            println!("{}", text);
            println!("--- --- ---");
        } else if &args[1] == "games" {
            let client_id = &args[2];
            let token = &args[3];
            let query = r#"fields id, game, name, category;
where id = (1866366,2639114,156334,15147);"#;
            let client = Client::new();
            let response = client
                .post("https://api.igdb.com/v4/external_games")
                .header("Client-ID", client_id)
                .header("Authorization", format!("Bearer {}", token))
                .body(query)
                .send()
                .await
                .expect("Failed to send request");

            let text = response.text().await.expect("Failed to read response body");
            println!("--- --- ---");
            println!("{}", text);
            println!("--- --- ---");
        } else if &args[1] == "prepare_external_games_list" {
            let expected_game_count = args[2]
                .parse::<u64>()
                .expect("Unable to parse expected_game_count");
            let steam_result_filename = &args[3];
            let text_raw = fs::read_to_string(steam_result_filename).expect("Failed to read file");
            let decoded_bytes: Vec<u8> =
                base64_url::decode(text_raw.trim()).expect("Error decoding base64url");
            let json: serde_json::Value =
                serde_json::from_slice(&decoded_bytes).expect("Failed to parse JSON");
            if let Some(game_count) = json
                .get("response")
                .and_then(|r| r.get("game_count"))
                .and_then(|v| v.as_u64())
            {
                if game_count == expected_game_count {
                    println!("game_count={}", game_count);
                    let mut appids = Vec::new();
                    for (_, game) in json["response"]["games"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .enumerate()
                    {
                        let appid = game["appid"].as_u64().unwrap();
                        appids.push(appid);
                    }
                    println!("appids={:?}", appids);
                    let batch_size = 10;
                    for (batch_index, chunk) in appids.chunks(batch_size).enumerate() {
                        let filename = format!("external_games_{}.txt", batch_index);
                        let mut file =
                            File::create(&filename).expect("Failed to create batch file");
                        for appid in chunk {
                            writeln!(file, "{}", appid).expect("Failed to write appid to file");
                        }
                        println!("Wrote {} ids to {}", chunk.len(), filename);
                    }
                } else {
                    println!(
                        "game_count invalid: got {}, expected {}",
                        game_count, expected_game_count
                    );
                }
            } else {
                println!("Failed to get game_count");
            }
        } else if &args[1] == "seek_external_games_list" {
            println!("(seek_external_games_list)");
            let filename = &args[2];
            let lines = read_lines(filename);
            let external_game_ids = lines
                .iter()
                .map(|id| id.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            println!("external_game_ids={:?}", external_game_ids);
        }

        //process_old_one();
        //process_old_two();
    }

    //steam_reqwest_logic();
    println!("<END>");
}

#[allow(dead_code)]
fn process_old_one() {
    //let steam_payload_json = args[1].clone();
    //let json_payload = fs::read_to_string(steam_payload_json).expect("Failed to read file");
    //let _ = process_steam_payload_json(&json_payload);
}

#[allow(dead_code)]
fn process_old_two() {
    //let steam_result_filename = args[1].clone();
    //println!("steam_result_filename={}", steam_result_filename);
    //process_steam_result(&steam_result_filename);
}

#[allow(dead_code)]
fn process_steam_payload_json(json_payload: &str) {
    println!("json.len={}", json_payload.len());
    let json: serde_json::Value = serde_json::from_str(json_payload).expect("Failed to parse JSON");
    if let Some(game_count) = json
        .get("response")
        .and_then(|r| r.get("game_count"))
        .and_then(|v| v.as_u64())
    {
        println!("game_count={}", game_count);
    } else {
        println!("Failed to get game_count");
    }
}

#[allow(dead_code)]
fn process_steam_result(steam_result_filename: &str) {
    let text_raw = fs::read_to_string(steam_result_filename).expect("Failed to read file");
    let text = text_raw.trim();
    let decoded = base64_url::decode(text).expect("Error decoding");
    println!("decoded.len={}", decoded.len());
    let filename = prepare_filename();
    fs::write(filename, &decoded).expect("Unable to write file");
}

#[allow(dead_code)]
#[tokio::main]
async fn steam_reqwest_logic() -> Result<(), reqwest::Error> {
    let loaded_toml = steam_load_toml().unwrap();
    println!(
        "player_id={}, key={}, url_raw={}",
        loaded_toml.player_id.len(),
        loaded_toml.key.len(),
        loaded_toml.url_raw.len()
    );
    let client = reqwest::Client::new();

    let url_with_player_id_replaced = loaded_toml
        .url_raw
        .as_str()
        .replace("{player_id}", &loaded_toml.player_id);
    let steam_url = url_with_player_id_replaced
        .as_str()
        .replace("{key}", &loaded_toml.key);

    let response = client.get(steam_url).send().await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let pretty_json = serde_json::to_string_pretty(&json).unwrap();
        let encoded_payload = base64_url::encode(&pretty_json);
        let filename = prepare_filename();
        fs::write(filename, &encoded_payload).expect("Unable to write file");
        println!("--- RAW JSON PAYLOAD ---");
        println!("{}", pretty_json);
        println!("-------------------------");
    } else {
        println!(
            "c9cf6abb Error=[{}][{}]",
            response.status(),
            response.text().await?
        );
    }

    Ok(())
}

//
// ---
//

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[allow(dead_code)]
#[tokio::main]
async fn old_youtube_api_reqwest_logic() -> Result<(), reqwest::Error> {
    let loaded_toml = seek_load_toml().unwrap();
    println!(
        "winter_2021={}, example_url={}, general_json_example_url={}, stereotypical_youtube_video_url_root={}, youtube_api_v3_base_url={}, youtube_api_v3_key_parts={}",
        loaded_toml.winter_2021.len(),
        loaded_toml.example_url.len(),
        loaded_toml.general_json_example_url.len(),
        loaded_toml.stereotypical_youtube_video_url_root.len(),
        loaded_toml.youtube_api_v3_base_url.len(),
        loaded_toml.youtube_api_v3_key_parts.len()
    );
    let client = reqwest::Client::new();
    let params = [
        ("part", loaded_toml.youtube_api_v3_key_parts),
        ("id", loaded_toml.example_url.clone()),
        ("key", loaded_toml.winter_2021),
    ];

    let response = client
        .get(loaded_toml.youtube_api_v3_base_url)
        .query(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let pretty_json = serde_json::to_string_pretty(&json).unwrap();
        let encoded_payload = base64_url::encode(&pretty_json);
        let filename = prepare_filename();
        fs::write(filename, &encoded_payload).expect("Unable to write file");
        println!("--- RAW JSON PAYLOAD ---");
        println!("{}", pretty_json);
        println!("-------------------------");
    } else {
        println!(
            "c9cf6abb Error=[{}][{}]",
            response.status(),
            response.text().await?
        );
    }

    Ok(())
}

fn seek_load_toml() -> Result<Youtube, String> {
    let content_raw = fs::read_to_string(r"C:\Users\igor\.ssh\youtube.toml")
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: YoutubeConfig =
        toml::from_str(&content_raw).map_err(|e| format!("Failed to parse TOML: {}", e))?;

    Ok(config.youtube)
}

fn steam_load_toml() -> Result<Steam, String> {
    let content_raw = fs::read_to_string(r"C:\Users\igor\.ssh\steam_oldschool.toml")
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: SteamConfig =
        toml::from_str(&content_raw).map_err(|e| format!("Failed to parse TOML: {}", e))?;

    Ok(config.steam)
}

fn prepare_filename() -> String {
    let now = chrono::Utc::now();
    let year = now.year();
    let month = now.month().to_string();
    let day = now.day().to_string();
    let formatted_month = zero_leading_format(&month);
    let formatted_day = zero_leading_format(&day);
    let uuid_substring = &Uuid::new_v4().to_string()[..8].to_uppercase();
    let filename = format!(
        "{}-{}-{}_{}.txt",
        year, formatted_month, formatted_day, uuid_substring
    );

    filename
}

fn zero_leading_format(input: &str) -> String {
    format!("{:0>2}", input)
}

#[allow(dead_code)]
fn stereotypical_youtube_video_url_valid(url: &str) -> bool {
    let pattern = r"^https://www\.youtube\.com/watch\?v=[a-zA-Z0-9_-]{11}$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(url)
}

#[cfg(test)]
mod tests {
    use crate::{
        read_lines, seek_load_toml, stereotypical_youtube_video_url_valid, zero_leading_format,
    };
    use chrono::Datelike;
    use std::fs;

    #[test]
    fn test_encode_hello_world() {
        assert_eq!("SGVsbG8sIHdvcmxkIQ", base64_url::encode("Hello, world!"));
    }

    #[test]
    fn test_decode_hello_world() {
        assert_eq!(
            b"Hello, world!",
            base64_url::decode("SGVsbG8sIHdvcmxkIQ").unwrap().as_slice()
        );
    }

    #[test]
    fn test_escape_base64_url() {
        assert_eq!(
            "SGVsbG8sIHdvcmxkIQ",
            base64_url::escape("SGVsbG8sIHdvcmxkIQ==")
        );
    }

    #[test]
    fn test_unescape_base64_url() {
        assert_eq!(
            "SGVsbG8sIHdvcmxkIQ==",
            base64_url::unescape("SGVsbG8sIHdvcmxkIQ")
        );
    }

    #[test]
    fn test_encode_to_string_appended_to_url() {
        let hash = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut url = String::from("https://example.com/?hash=");

        assert_eq!("AQIDBAUGBwgJ", base64_url::encode_to_string(hash, &mut url));
        assert_eq!("https://example.com/?hash=AQIDBAUGBwgJ", url);
    }

    #[test]
    fn test_current_year() {
        let now = chrono::Utc::now();
        let year = now.year();
        assert_eq!("2026", year.to_string());
    }

    #[test]
    fn test_current_month() {
        let now = chrono::Utc::now();
        let month = now.month().to_string();
        let formatted_month = zero_leading_format(&month);
        assert_eq!("02", formatted_month);
    }

    #[test]
    fn test_current_day() {
        let now = chrono::Utc::now();
        let day = now.day().to_string();
        let formatted_day = zero_leading_format(&day);
        assert_eq!("10", formatted_day);
    }

    #[test]
    fn test_encoded_payload_matches_decoded_payload() {
        let encoded_payload = include_str!("../example_encoded_payload.txt").trim();
        let decoded_payload = include_str!("../example_decoded_payload.txt")
            .trim()
            .replace("\r\n", "\n");
        assert_eq!(base64_url::encode(&decoded_payload), encoded_payload);
    }

    #[test]
    fn test_stereotypical_youtube_video_url() {
        let loaded_toml = seek_load_toml().unwrap();
        let url = format!(
            "{}{}",
            loaded_toml.stereotypical_youtube_video_url_root, loaded_toml.example_url
        );
        assert_eq!(
            stereotypical_youtube_video_url_valid(r"https://www.youtube.com/watch?v=Žs15wnpm9mGY"),
            false
        );
        assert_eq!(
            stereotypical_youtube_video_url_valid(r"https://www.youtube.com/watch?v=s15wnpm9mGYŽ"),
            false
        );
        assert_eq!(stereotypical_youtube_video_url_valid(&url), true);
    }

    #[test]
    fn filter_one() {
        let january = r"C:\D\notes\january";
        let mut count_all = 0;
        let mut count_valid = 0;
        fs::read_dir(january)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "txt"))
            .for_each(|file_content| {
                for content in fs::read_to_string(file_content).unwrap().lines() {
                    if stereotypical_youtube_video_url_valid(content) {
                        count_valid += 1;
                    }
                    count_all += 1;
                }
            });
        assert_eq!(count_valid, 444);
        assert_eq!(count_all, 1833);
    }

    #[test]
    fn filter_two() {
        let mut basic_count = 0;
        let mut advanced_count = 0;
        let lines = read_lines("strict_url_examples.txt");
        let lines_iterable = lines.clone();
        lines_iterable.into_iter().for_each(|line| {
            if line.len() > 0 {
                basic_count += 1;
            }
        });
        lines.into_iter().for_each(|line| {
            if line.len() > 0 {
                if stereotypical_youtube_video_url_valid(&line) {
                    advanced_count += 1;
                }
            }
        });
        assert_eq!(advanced_count, 1);
        assert_eq!(basic_count, 5);
    }
}
