use chrono::Datelike;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Deserialize)]
struct Youtube {
    winter_2021: String,
    example_url: String,
    general_json_example_url: String,
    youtube_api_v3_base_url: String,
    youtube_api_v3_key_parts: String,
}

#[derive(Clone, Deserialize)]
struct Config {
    youtube: Youtube,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let loaded_toml = seek_load_toml().unwrap();
    println!(
        "winter_2021={}, example_url={}, general_json_example_url={}, youtube_api_v3_base_url={}, youtube_api_v3_key_parts={}",
        loaded_toml.winter_2021.len(),
        loaded_toml.example_url.len(),
        loaded_toml.general_json_example_url.len(),
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
        std::fs::write(filename, &encoded_payload).expect("Unable to write file");
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
    let content_raw = std::fs::read_to_string(r"C:\Users\igor\.ssh\youtube.toml")
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: Config =
        toml::from_str(&content_raw).map_err(|e| format!("Failed to parse TOML: {}", e))?;

    Ok(config.youtube)
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

#[cfg(test)]
mod tests {
    use crate::zero_leading_format;
    use chrono::Datelike;

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
        assert_eq!("01", formatted_month);
    }

    #[test]
    fn test_current_day() {
        let now = chrono::Utc::now();
        let day = now.day().to_string();
        let formatted_day = zero_leading_format(&day);
        assert_eq!("21", formatted_day);
    }

    #[test]
    fn test_encoded_payload_matches_decoded_payload() {
        let encoded_payload = include_str!("../example_encoded_payload.txt").trim();
        let decoded_payload = include_str!("../example_decoded_payload.txt")
            .trim()
            .replace("\r\n", "\n");
        assert_eq!(base64_url::encode(&decoded_payload), encoded_payload);
    }
}
