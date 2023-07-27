use std::env;
use seven_client::client::Client;

pub const SENT_WITH: &str = "Rust";

pub fn get_api_key() -> String {
    return env::var("SEVEN_API_KEY_SANDBOX").unwrap();
}

pub fn get_client() -> Client {
    return Client::new(get_api_key().to_string(), SENT_WITH.to_string());
}
