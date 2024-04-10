use std::env;
use rand::distributions::Alphanumeric;
use seven_client::client::Client;
use rand::{thread_rng, Rng};

pub const SENT_WITH: &str = "Rust";

pub fn get_api_key() -> String {
    return env::var("SEVEN_API_KEY").unwrap();
}

pub fn get_client() -> Client {
    return Client::new(get_api_key().to_string(), SENT_WITH.to_string());
}

#[cfg(test)]
pub fn rand_str(n: usize) -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect();
}