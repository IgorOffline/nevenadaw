use chrono::Datelike;
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GameThreeBody {
    response: GameThreeResponse,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GameThreeResponse {
    game_count: u32,
    games: Vec<GameThreeGame>,
}

#[derive(Debug, Deserialize)]
struct GameThreeGame {
    appid: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RootFour {
    id: u32,
    game: GameFour,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GameFour {
    url: String,
    total_rating: Option<f32>,
}

fn main() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    println!("args.len={}", args.len());
    if args.len() >= 3 && args[1] == "process_1070" {
        println!("(process_1070)");
        let directories = &args[2..];
        let mut highly_rated_games: Vec<GameFour> = Vec::new();
        for directory in directories {
            let mut games = process_directory(&directory);
            highly_rated_games.append(&mut games);
        }
        println!("highly_rated_games.len={}", highly_rated_games.len());
        highly_rated_games.sort_by(|a, b| {
            b.total_rating
                .partial_cmp(&a.total_rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for game in highly_rated_games.iter() {
            println!(
                "url={}, total_rating={}",
                game.url,
                game.total_rating.unwrap()
            );
        }
    }

    println!("<END>");
}

fn process_directory(directory: &&String) -> Vec<GameFour> {
    let mut return_value: Vec<GameFour> = Vec::new();
    if Path::new(directory).exists() == false {
        println!("Directory {} does not exist", directory);
        return return_value;
    }
    fs::read_dir(directory)
        .expect("Failed to read directory")
        .for_each(|entry| {
            let path = entry.unwrap().path();
            let raw = fs::read_to_string(path).expect("Failed to read file");
            let games: Vec<RootFour> = serde_json::from_str(&raw).expect("Parsing failed");
            if games.len() > 0 {
                for game in games {
                    let total_rating = game.game.total_rating.unwrap_or_else(|| 0.0);
                    //println!("url={}, total_rating={}", game.game.url, total_rating);
                    if total_rating > 69.0 {
                        let new_game: GameFour = GameFour {
                            url: game.game.url.clone(),
                            total_rating: Some(total_rating),
                        };
                        return_value.push(new_game);
                    }
                }
            }
        });

    return_value
}

#[allow(dead_code)]
#[tokio::main]
async fn main_seek_1070() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len == 5 && &args[1] == "seek_1070" {
        println!("(seek_1070)");
        let game_ids_path = &args[2];
        let raw = fs::read_to_string(game_ids_path).expect("Failed to read id file");
        let games: Vec<GameThreeGame> = raw
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u32>().expect("Bad appid line"))
            .map(|appid| GameThreeGame { appid })
            .collect();
        println!("games.len={}", games.len());
        let client = Client::new();
        let client_id = &args[3];
        let token = &args[4];
        process_games_slice(&client, client_id, token, &games).await;
    }
    println!("<END>");
}

#[allow(dead_code)]
async fn process_games_slice(
    client: &Client,
    client_id: &String,
    token: &String,
    games_slice: &[GameThreeGame],
) {
    for game in games_slice {
        let game_id = game.appid;
        println!("game_id={}", game_id);
        let query = format!(
            r#"fields game.*; where external_game_source = 1 & uid = "{}";"#,
            game_id
        );
        let response = client
            .post("https://api.igdb.com/v4/external_games")
            .header("Client-ID", client_id)
            .header("Authorization", format!("Bearer {}", token))
            .body(query)
            .send()
            .await
            .expect("Failed to send request");

        let body = response.text().await.expect("Failed to read response body");
        println!("body.len={}", body.len());

        let filename = prepare_filename();
        fs::write(filename, body).expect("Unable to write file");

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
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
