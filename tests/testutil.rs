use std::env;
use sms77_client::client::Client;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub const SENT_WITH: &str = "Rust";

pub fn get_api_key() -> String {
    return env::var("SMS77_DUMMY_API_KEY").unwrap();
}

pub fn get_client() -> Client {
    return Client::new(get_api_key().to_string(), SENT_WITH.to_string());
}

pub fn rand_str() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    rand_string
}