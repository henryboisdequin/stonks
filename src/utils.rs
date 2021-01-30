use dotenv::dotenv;
use std::env;

pub fn get_api_key() -> String {
    dotenv().ok();
    let key = "API_KEY";
    env::var(key).expect("API key not present in `.env` file.")
}
