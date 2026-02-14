use chrono::Datelike;
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use uuid::Uuid;

use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};

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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GameOne {
    id: u64,
    game: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GameTwo {
    id: u64,
    url: String,
}

#[derive(Debug, Deserialize)]
struct GameThreeBody {
    response: GameThreeResponse,
}

#[derive(Debug, Deserialize)]
struct GameThreeResponse {
    game_count: u32,
    games: Vec<GameThreeGame>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GameThreeGame {
    appid: u32,
}

fn main() {
    println!("<START>");
    println!("<END>");
}

#[allow(dead_code)]
#[tokio::main]
async fn main_game_batches_1060() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len == 3 && &args[1] == "seek_1060" {
        println!("(seek_1060)");
        let games_json_path = &args[2];
        let json_raw = fs::read_to_string(games_json_path).expect("Failed to read JSON file");
        let json_bytes = base64_url::decode(&json_raw).expect("Error decoding base64");
        let json_string = String::from_utf8(json_bytes).expect("Decoded data is not valid UTF-8");
        let mut games =
            serde_json::from_str::<GameThreeBody>(&json_string).expect("Failed to parse JSON");
        println!("--- --- ---");
        println!("{:?}", games);
        println!("--- --- ---");
        let mut rng = StdRng::seed_from_u64(12345);
        games.response.games.shuffle(&mut rng);
        assert_eq!(games.response.games.len() > 10, true);
        let batch_size = 30;
        for (batch_idx, chunk) in games.response.games.chunks(batch_size).enumerate() {
            let filename = format!("game_batch_{:03}.txt", batch_idx);
            let file = File::create(&filename).expect("Failed to create batch file");
            let mut writer = BufWriter::new(file);
            for game in chunk {
                writeln!(writer, "{}", game.appid).expect("Failed to write appid");
            }
            writer.flush().ok();
            println!("wrote {} appids -> {}", chunk.len(), filename);
        }
    }
    println!("<END>");
}

#[allow(dead_code)]
#[tokio::main]
async fn main_old_ten_fifty() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len == 5 && &args[1] == "seek_1050" {
        println!("(seek_1050)");
        let games_json_path = &args[2];
        let json_raw = fs::read_to_string(games_json_path).expect("Failed to read JSON file");
        let json_bytes = base64_url::decode(&json_raw).expect("Error decoding base64");
        let json_string = String::from_utf8(json_bytes).expect("Decoded data is not valid UTF-8");
        let mut games =
            serde_json::from_str::<GameThreeBody>(&json_string).expect("Failed to parse JSON");
        println!("--- --- ---");
        println!("{:?}", games);
        println!("--- --- ---");
        let mut rng = StdRng::seed_from_u64(12345);
        games.response.games.shuffle(&mut rng);
        assert_eq!(games.response.games.len() > 10, true);
        let ten = &games.response.games[..10.min(games.response.games.len())];
        println!("ten={:?}", ten);
        let client = Client::new();
        let client_id = &args[3];
        let token = &args[4];
        process_games_slice_old_ten_fifty(&client, client_id, token, ten).await;
    }
    println!("<END>");
}

#[allow(dead_code)]
async fn process_games_slice_old_ten_fifty(
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

#[allow(dead_code)]
#[tokio::main]
async fn main_err_slice() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len == 5 && &args[1] == "seek_1040" {
        println!("(seek_1040)");
        let games_json_path = &args[2];
        let json_raw = fs::read_to_string(games_json_path).expect("Failed to read JSON file");
        println!("content.len={}", &json_raw.len());
        let mut games =
            serde_json::from_str::<GameThreeBody>(&json_raw).expect("Failed to parse JSON");
        println!(
            "games.response.game_count={}, games.response.games.len={}",
            games.response.game_count,
            games.response.games.len()
        );
        assert_eq!(
            games.response.game_count,
            games.response.games.len() as u32,
            "games.response.game_count != games.response.games.len()"
        );
        let mut rng = StdRng::seed_from_u64(12345);
        games.response.games.shuffle(&mut rng);
        assert_eq!(games.response.games.len() > 10, true);
        let ten = &games.response.games[..10.min(games.response.games.len())];
        println!("ten={:?}", ten);
        let client = Client::new();
        let client_id = &args[3];
        let token = &args[4];
        process_games_slice_old(&client, client_id, token, ten).await;
    }
    println!("<END>");
}

#[allow(dead_code)]
async fn process_games_slice_old(
    client: &Client,
    client_id: &String,
    token: &String,
    games_slice: &[GameThreeGame],
) {
    for game in games_slice {
        let game_id = game.appid;
        println!("game_id={}", game_id);
        let query = format!(
            "fields age_ratings,aggregated_rating,aggregated_rating_count,alternative_names,artworks,bundles,category,checksum,collection,collections,cover,created_at,dlcs,expanded_games,expansions,external_games,first_release_date,follows,forks,franchise,franchises,game_engines,game_localizations,game_modes,game_status,game_type,genres,hypes,involved_companies,keywords,language_supports,multiplayer_modes,name,parent_game,platforms,player_perspectives,ports,rating,rating_count,release_dates,remakes,remasters,screenshots,similar_games,slug,standalone_expansions,status,storyline,summary,tags,themes,total_rating,total_rating_count,updated_at,url,version_parent,version_title,videos,websites; where id = {};",
            game_id
        );
        let response = client
            .post("https://api.igdb.com/v4/games")
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

#[allow(dead_code)]
fn main_analyze_one_game_1002() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len == 3 && &args[1] == "analyze_one_game_1002" {
        println!("(analyze_one_game_1002)");
        let lines = read_lines(&args[2]);
        let mut batch_path_list: Vec<String> = Vec::new();
        lines.iter().for_each(|batch_path| {
            let exists = Path::new(&batch_path).exists();
            if !exists {
                panic!("File not found: {}", batch_path);
            }
            println!("batch_path={}, exists={}", batch_path, exists);
            batch_path_list.push(batch_path.to_string());
        });
        let games = process_batch_path_list(&batch_path_list);
        process_games(&games);
    } else {
        println!("ERR::args args_len={}", args_len);
    }
    println!("<END>");
}

#[allow(dead_code)]
fn process_batch_path_list(batch_path_list: &Vec<String>) -> Vec<GameTwo> {
    let mut games: Vec<GameTwo> = Vec::new();
    for batch_dir in batch_path_list {
        let batch_path = Path::new(batch_dir);
        if !batch_path.is_dir() {
            panic!("batch_path is not a directory: {}", batch_path.display());
        }
        for entry_raw in fs::read_dir(batch_path).expect("Failed to read batch directory") {
            let entry = entry_raw.expect("Bad entry");
            let file_path = entry.path();
            if !file_path.is_file() {
                panic!("Not a file: {}", file_path.display());
            }
            let content = fs::read_to_string(&file_path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", file_path.display(), e));
            let games_local: Vec<GameTwo> = serde_json::from_str(&content).unwrap_or_else(|e| {
                panic!("Failed to parse JSON in {}: {}", file_path.display(), e)
            });
            games.extend(games_local);
        }
    }

    games
}

#[allow(dead_code)]
fn process_games(games: &Vec<GameTwo>) {
    println!("--- --- ---");
    for game in games {
        println!("{}", game.url);
    }
}

//
// --- --- ---
//

#[allow(dead_code)]
fn main_analyze_one_game_1001() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len > 1 && &args[1] == "analyze_one_game_1001" {
        println!("(analyze_one_game_1001)");
        let paths: Vec<String> = serde_json::from_str(&args[2]).expect("Failed to parse JSON");
        println!("paths={:?}", paths);
        for path in &paths {
            let exists = Path::new(&path).exists();
            if !exists {
                panic!("File not found: {}", path);
            }
        }
        let mut content_list: Vec<String> = Vec::new();
        for path in paths {
            for entry in fs::read_dir(&path).expect("Failed to read directory") {
                let file_path = entry.expect("Bad entry").path();

                if !file_path.is_file() {
                    panic!("Not a file: {}", file_path.display());
                }

                let content = fs::read_to_string(&file_path).expect("Failed to read file");
                content_list.push(content);
            }
        }
        println!("content_list.len={}", content_list.len());
        println!("--- --- ---");
        for content in content_list {
            let game: GameTwo = serde_json::from_str(&content).expect("Failed to parse JSON");
            println!("{:?}", game.url);
        }
        println!("--- --- ---");
    } else {
        println!("ERR::args args_len={}", args_len);
    }
    println!("<END>");
}

#[allow(dead_code)]
fn main_analyze_one_game_1000() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len > 1 && &args[1] == "analyze_one_game_1000" {
        println!("(analyze_one_game_1000)");
        let game_filename = &args[2];
        let prepared_json_raw = fs::read_to_string(game_filename).expect("Failed to read file");
        let games: Vec<GameTwo> =
            serde_json::from_str(&prepared_json_raw).expect("Failed to parse JSON");
        //println!("games={:?}", games);
        let mut some_count = 0;
        games.iter().for_each(|game| {
            if game.url.len() > 0 {
                some_count += 1;
            }
        });
        println!("some_count={}", some_count);
    } else {
        println!("ERR::args args_len={}", args_len);
    }
    println!("<END>");
}

#[allow(dead_code)]
#[tokio::main]
async fn main_process_game_one_old() {
    println!("<START>");
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    println!("args.len={:?}", args_len);
    if args_len > 1 && &args[1] == "seek_game_batch" {
        let prepared_json_raw =
            fs::read_to_string(&args[2]).expect("Failed to read file to string");
        let games: Vec<GameOne> =
            serde_json::from_str(&prepared_json_raw).expect("Failed to parse JSON");
        println!("games={:?}", games);
        let client = Client::new();
        for game in games {
            let client_id = &args[3];
            let token = &args[4];
            process_game_one(client_id, token, &client, &game).await;
        }
    } else {
        println!("Please provide valid arguments");
    }
    println!("<END>");
}

async fn process_game_one(client_id: &str, token: &str, client: &Client, game: &GameOne) {
    let game_id = game.game;
    let query = format!(
        "fields age_ratings,aggregated_rating,aggregated_rating_count,alternative_names,artworks,bundles,category,checksum,collection,collections,cover,created_at,dlcs,expanded_games,expansions,external_games,first_release_date,follows,forks,franchise,franchises,game_engines,game_localizations,game_modes,game_status,game_type,genres,hypes,involved_companies,keywords,language_supports,multiplayer_modes,name,parent_game,platforms,player_perspectives,ports,rating,rating_count,release_dates,remakes,remasters,screenshots,similar_games,slug,standalone_expansions,status,storyline,summary,tags,themes,total_rating,total_rating_count,updated_at,url,version_parent,version_title,videos,websites; where id = {};",
        game_id
    );
    let response = client
        .post("https://api.igdb.com/v4/games")
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

#[allow(dead_code)]
#[tokio::main]
async fn initial_arguments() {
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
            let external_game_ids_stringified_raw = external_game_ids
                .iter()
                .map(|id| format!("\"{}\"", id))
                .collect::<Vec<_>>()
                .join(",");

            let external_game_ids_stringified = format!("({})", external_game_ids_stringified_raw);
            println!(
                "external_game_ids={:?} ; [{}]",
                external_game_ids, external_game_ids_stringified
            );

            let client_id = &args[3];
            let token = &args[4];
            let query = format!(
                r#"fields game; where uid = {} & external_game_source = 1;"#,
                external_game_ids_stringified
            );
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
        } else if &args[1] == "seek_one_game" {
            let client_id = &args[2];
            let token = &args[3];
            let game_id = "14";
            let query = format!(
                "fields age_ratings,aggregated_rating,aggregated_rating_count,alternative_names,artworks,bundles,category,checksum,collection,collections,cover,created_at,dlcs,expanded_games,expansions,external_games,first_release_date,follows,forks,franchise,franchises,game_engines,game_localizations,game_modes,game_status,game_type,genres,hypes,involved_companies,keywords,language_supports,multiplayer_modes,name,parent_game,platforms,player_perspectives,ports,rating,rating_count,release_dates,remakes,remasters,screenshots,similar_games,slug,standalone_expansions,status,storyline,summary,tags,themes,total_rating,total_rating_count,updated_at,url,version_parent,version_title,videos,websites; where id = {};",
                game_id
            );
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
