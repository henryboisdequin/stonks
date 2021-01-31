use serde::{Deserialize, Serialize};
use std::fs;
use toml::from_str;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub viewing: Vec<Stocks>,
}

#[derive(Serialize, Deserialize)]
pub struct Stocks {
    pub symbol: String,
}

pub fn parse_toml_file(path: String) -> Config {
    let content: String = fs::read_to_string(path)
        .expect("Failed to access file.")
        .parse()
        .expect("Failed to parse file.");

    let toml_config: Config = from_str(&content[..]).expect("Failed to parse file.");

    toml_config
}
