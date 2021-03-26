use serde::{Deserializer, de, Deserialize};
use serde_json::Value;

pub mod analytics;
pub mod balance;
pub mod client;
pub mod journal;
pub mod lookup;
pub mod pricing;
pub mod sms;
pub mod status;
pub mod voice;
pub mod hooks;
pub mod validate_for_voice;

fn to_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => return Err(de::Error::custom("wrong type"))
    })
}