use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Root {
    lorem: String,
    ipsum: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    root: Root,
}

fn main() -> Result<(), String> {
    let mut content_raw = String::new();
    File::open("sdl2later.toml")
        .unwrap()
        .read_to_string(&mut content_raw)
        .expect("I'm learning, give me a break!");
    let content = content_raw.clone();

    println!("SDL2 Later; Load TOML first :)");
    let config: Config = toml::from_str(&content).unwrap();
    println!(
        "{:#?}\n\n{}\n\n{}",
        config, config.root.lorem, config.root.ipsum
    );
    Ok(())
}
