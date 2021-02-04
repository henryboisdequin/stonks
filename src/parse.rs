use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use toml;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub symbols: Vec<String>,
}

// #[derive(Deserialize, Debug)]
// pub struct Watching {
//     pub symbols: Vec<Stocks>,
// }

// #[derive(Deserialize, Debug)]
// pub struct Stocks {
//     pub symbol: String,
// }

pub fn parse_toml_file<'a>(path: String) -> Vec<Config> {
    let content: &str = &fs::read_to_string(path).expect("Failed to access file.");
    let config_hash_map: HashMap<String, Vec<Config>> = toml::from_str(content).unwrap();

    // let toml_config: Config = toml::from_str(&content[..]).expect("Failed to parse file.");
    let toml_config: &[Config] = &config_hash_map["watching"];

    toml_config.to_vec()

    // Config {
    //     watching: vec![Stocks {
    //         symbol: "".to_string(),
    //     }],
    // }
}
