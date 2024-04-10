use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize};

const ENDPOINT: &str = "sms";

pub struct SmsParams {
    pub delay: Option<String>,
    pub flash: Option<bool>,
    pub foreign_id: Option<String>,
    pub from: Option<String>,
    pub label: Option<String>,
    pub text: String,
    pub to: String,
    pub udh: Option<String>,
    pub ttl: Option<u32>,
    pub performance_tracking: Option<bool>,
}

#[derive(Deserialize)]
pub struct SmsResponse {
    pub balance: f64,
    pub debug: String,
    pub messages: Vec<SmsMessage>,
    pub sms_type: String,
    pub success: String,
    pub total_price: f64,
}

#[derive(Deserialize)]
pub struct SmsMessage {
    pub encoding: String,
    pub error: Option<u16>,
    pub error_text: Option<String>,
    pub id: Option<String>,
    pub is_binary: bool,
    pub label: Option<String>,
    pub messages: Option<Vec<String>>,
    pub parts: u8,
    pub price: f64,
    pub recipient: String,
    pub sender: String,
    pub success: bool,
    pub text: String,
    pub udh: Option<String>,
}

pub struct Sms {
    client: Client
}

impl Sms {
    pub fn new(client: Client) -> Self {
        Sms {
            client,
        }
    }

    pub fn dispatch(&self, params: SmsParams) -> Result<SmsResponse, Error> {
        Ok(self.client.post(ENDPOINT)
            .send_form(&[
                ("delay", &*params.delay.unwrap_or_default()),
                ("flash", &*params.flash.unwrap_or_default().to_string()),
                ("foreign_id", &*params.foreign_id.unwrap_or_default()),
                ("from", &*params.from.unwrap_or_default()),
                ("label", &*params.label.unwrap_or_default()),
                ("performance_tracking", &*params.performance_tracking.unwrap_or_default().to_string()),
                ("text", &*params.text),
                ("to", &*params.to),
                ("ttl", &*params.ttl.unwrap_or_default().to_string()),
                ("udh", &*params.udh.unwrap_or_default()),
            ])?
            .into_json::<SmsResponse>()?)
    }
}