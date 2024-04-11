use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
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

pub fn current_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
}

pub fn create_mail() -> String {
    format!("rust_{}@seven.dev", current_timestamp()).to_string()
}