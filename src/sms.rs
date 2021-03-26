use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize};

const ENDPOINT: &str = "sms";

pub struct SmsJsonParams {
    pub debug: Option<bool>,
    pub delay: Option<String>,
    pub flash: Option<bool>,
    pub foreign_id: Option<String>,
    pub from: Option<String>,
    pub label: Option<String>,
    pub no_reload: Option<bool>,
    pub text: String,
    pub to: String,
    pub unicode: Option<bool>,
    pub udh: Option<String>,
    pub utf8: Option<bool>,
    pub ttl: Option<u32>,
    pub performance_tracking: Option<bool>,
}

#[derive(Deserialize)]
pub struct SmsJsonResponse {
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
    pub error: Option<String>,
    pub error_text: Option<String>,
    pub id: Option<String>,
    pub messages: Option<Vec<String>>,
    pub parts: u16,
    pub price: f64,
    pub recipient: String,
    pub sender: String,
    pub success: bool,
    pub text: String,
}

pub struct SmsTextParams {
    pub debug: Option<bool>,
    pub delay: Option<String>,
    pub details: Option<bool>,
    pub flash: Option<bool>,
    pub foreign_id: Option<String>,
    pub from: Option<String>,
    pub label: Option<String>,
    pub no_reload: Option<bool>,
    pub text: String,
    pub to: String,
    pub unicode: Option<bool>,
    pub udh: Option<String>,
    pub utf8: Option<bool>,
    pub ttl: Option<u32>,
    pub performance_tracking: Option<bool>,
    pub return_msg_id: Option<bool>,
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

    pub fn text(&self, params: SmsTextParams) -> Result<String, Error> {
        Ok(self.client.request("POST", ENDPOINT)
            .send_form(&[
                ("debug", self.client.bool_to_string(params.debug.unwrap_or_default())),
                ("delay", &*params.delay.unwrap_or_default()),
                ("details", self.client.bool_to_string(params.details.unwrap_or_default())),
                ("flash", self.client.bool_to_string(params.flash.unwrap_or_default())),
                ("foreign_id", &*params.foreign_id.unwrap_or_default()),
                ("from", &*params.from.unwrap_or_default()),
                ("label", &*params.label.unwrap_or_default()),
                ("no_reload", self.client.bool_to_string(params.no_reload.unwrap_or_default())),
                ("performance_tracking", self.client.bool_to_string(params.performance_tracking.unwrap_or_default())),
                ("return_msg_id", self.client.bool_to_string(params.return_msg_id.unwrap_or_default())),
                ("text", &*params.text),
                ("to", &*params.to),
                ("ttl", &*params.ttl.unwrap_or_default().to_string()),
                ("udh", &*params.udh.unwrap_or_default()),
                ("unicode", self.client.bool_to_string(params.unicode.unwrap_or_default())),
                ("utf8", self.client.bool_to_string(params.utf8.unwrap_or_default())),
            ])?
            .into_string()?)
    }

    pub fn json(&self, params: SmsJsonParams) -> Result<SmsJsonResponse, Error> {
        Ok(self.client.request("POST", ENDPOINT)
            .send_form(&[
                ("debug", self.client.bool_to_string(params.debug.unwrap_or_default())),
                ("delay", &*params.delay.unwrap_or_default()),
                ("flash", self.client.bool_to_string(params.flash.unwrap_or_default())),
                ("foreign_id", &*params.foreign_id.unwrap_or_default()),
                ("from", &*params.from.unwrap_or_default()),
                ("json", self.client.bool_to_string(true)),
                ("label", &*params.label.unwrap_or_default()),
                ("no_reload", self.client.bool_to_string(params.no_reload.unwrap_or_default())),
                ("performance_tracking", self.client.bool_to_string(params.performance_tracking.unwrap_or_default())),
                ("text", &*params.text),
                ("to", &*params.to),
                ("ttl", &*params.ttl.unwrap_or_default().to_string()),
                ("udh", &*params.udh.unwrap_or_default()),
                ("unicode", self.client.bool_to_string(params.unicode.unwrap_or_default())),
                ("utf8", self.client.bool_to_string(params.utf8.unwrap_or_default())),
            ])?
            .into_json::<SmsJsonResponse>()?)
    }
}