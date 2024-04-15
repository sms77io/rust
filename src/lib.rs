use serde::{Deserializer, de, Deserialize, Serialize};
use serde_json::Value;

pub mod analytics;
pub mod balance;
pub mod client;
pub mod journal;
pub mod lookup;
pub mod pricing;
pub mod sms;
pub mod status;
pub mod subaccounts;
pub mod voice;
pub mod hooks;
pub mod validate_for_voice;
pub mod contacts;
pub mod rcs;
pub mod groups;
pub mod numbers;

#[derive(Debug, Deserialize)]
pub struct PagingMetadata {
    pub count: u64,
    pub has_more: bool,
    pub limit: u16,
    pub offset: u64,
    pub total: u64,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    #[default]
    Asc,
    Desc
}

fn to_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => return Err(de::Error::custom("wrong type"))
    })
}
