use super::client;
use super::parse::Config;
use super::utils::rem_first_and_last_char;
use finnhub_rs::types::CompanyQuote;
use std::collections::HashMap;

/// Where the config is stored.
pub struct Cli {
    /// Config inside $HOME/.config/stonks.toml.
    pub config: Vec<Config>,
}

impl Cli {
    /// Creates a new Cli.
    pub fn new(config: Vec<Config>) -> Self {
        Self { config }
    }

    /// Lists all the current symbols of the companies listed in config.
    pub fn list(self) {
        let config = self.config;

        println!("All the symbols of the companies you are watching:");

        for watching in config {
            for symbol in watching.symbols {
                println!("\t• {}", symbol);
            }
        }
    }

    /// Adds a symbol to the watching section in your config.
    pub fn add(self) {}

    /// Views the current stock price of all stocks you are watching in your config.
    pub async fn view<'a>(&self) {
        let mut hashmap: HashMap<String, CompanyQuote> = HashMap::new();
        let mut symbols = Vec::new();
        for watching in self.config.clone() {
            for symbol in &watching.symbols {
                let client = client::create_client();
                let value = client.quote(symbol.clone()).await.unwrap();
                hashmap.insert(symbol.clone(), value);
                symbols.push(symbol.clone());
            }
        }

        for symbol in symbols {
            match hashmap.get(&symbol) {
                Some(val) => {
                    println!("{}'s current stock value is ${}", symbol, val.c);
                }
                _ => {}
            }
        }
    }
}
