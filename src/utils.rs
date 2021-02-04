use dotenv::dotenv;
use std::env;

pub fn get_api_key() -> String {
    dotenv().ok();
    let key = "API_KEY";
    env::var(key).expect("API key not present in `.env` file.")
}

pub fn rem_first_and_last_char(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
