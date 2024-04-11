use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize, Serialize};

const ENDPOINT_MESSAGES: &str = "rcs/messages";

#[derive(Serialize)]
pub enum RcsEvent {
    #[serde(rename = "IS_TYPING")]
    IsTyping,
    #[serde(rename = "READ")]
    Read,
}

impl RcsEvent {
    fn as_str(&self) -> &'static str {
        match self {
            RcsEvent::IsTyping => "IS_TYPING",
            RcsEvent::Read => "READ",
        }
    }
}

#[derive(Serialize)]
pub struct RcsDeleteParams {
    pub id: String,
}

#[derive(Serialize)]
pub enum RcsEventTarget {
    #[serde(rename = "msg_id")]
    MessageId,
    #[serde(rename = "to")]
    PhoneNumber,
}

#[derive(Serialize)]
pub struct RcsEventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    msg_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    pub event: RcsEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing)]
    pub target: RcsEventTarget,
}

impl RcsEventParams {
    pub fn new(target: RcsEventTarget, value: String) -> Self {
        let mut params = Self{
            msg_id: None,
            to: None,
            event: RcsEvent::IsTyping,
            from: None,
            target,
        };

        let option = Option::from(value);

        match params.target {
            RcsEventTarget::MessageId => {
                params.msg_id = option;
            }
            RcsEventTarget::PhoneNumber => {
                params.to = option;
            }
        }

        params
    }
}

#[derive(Serialize)]
struct EventParams {
    #[serde(flatten)]
    msg_id: RcsEventParams,
}

#[derive(Serialize)]
pub struct RcsDispatchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub text: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_tracking: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct RcsDeleteResponse {
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct RcsEventResponse {
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct RcsResponse {
    pub balance: f64,
    pub debug: String,
    pub messages: Vec<RcsMessage>,
    pub sms_type: String,
    pub success: String,
    pub total_price: f64,
}

#[derive(Deserialize, Debug)]
pub struct RcsMessage {
    pub channel: String,
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

pub struct Rcs {
    client: Client,
}

impl Rcs {
    pub fn new(client: Client) -> Self {
        Rcs {
            client,
        }
    }

    pub fn delete(&self, params: RcsDeleteParams) -> Result<RcsDeleteResponse, Error> {
        let endpoint = format!("{}/{}", ENDPOINT_MESSAGES, params.id);

        Ok(self.client.delete(&*endpoint)
            .call()
            .unwrap()
            .into_json()?)
    }

    pub fn event(&self, params: RcsEventParams) -> Result<RcsEventResponse, Error> {
        Ok(self.client.post("rcs/events")
            .send_json(params)?
            .into_json()?)
    }

    pub fn dispatch(&self, params: RcsDispatchParams) -> Result<RcsResponse, Error> {
        Ok(self.client.post(ENDPOINT_MESSAGES)
            .send_json(params)?
            .into_json()?)
    }
}