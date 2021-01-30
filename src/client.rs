use super::utils::get_api_key;
use finnhub_rs::client;

pub fn create_client() -> client::Client {
    client::Client::new(get_api_key())
}
