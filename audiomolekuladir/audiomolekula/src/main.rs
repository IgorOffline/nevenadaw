use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, Debug, Deserialize)]
struct Regina {
    lorem: u32,
    ipsum: u32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct Config {
    regina: Regina,
}

impl std::fmt::Display for Regina {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}lorem={}, ipsum={}{}",
            r"Regina{", self.lorem, self.ipsum, r"}"
        )
    }
}

fn main() {
    println!("<START>");
    let coin_toss = rand::random_range(1..=2);
    let regina = audiomolekula_load_toml().expect("aafcf099 Failed to load audiomolekula config");
    println!("coin_toss= {}", coin_toss);
    println!("{}", regina);
    println!("<END>");
}

fn audiomolekula_load_toml() -> Result<Regina, String> {
    let mut content_raw = String::new();
    File::open("regina.toml")
        .map_err(|e| format!("1ec66bc5 Failed to open config file: {}", e))?
        .read_to_string(&mut content_raw)
        .map_err(|e| format!("eb77bacc Failed to read config file: {}", e))?;

    let config: Config = toml::from_str(&content_raw)
        .map_err(|e| format!("dd76e3df Failed to parse TOML: {}", e))?;

    let return_value = Regina {
        lorem: config.regina.lorem,
        ipsum: config.regina.ipsum,
    };

    Ok(return_value)
}
