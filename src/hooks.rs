use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

const ENDPOINT: &str = "hooks";

#[derive(Debug, Deserialize, EnumIter, EnumString, strum_macros::Display)]
pub enum EventType {
    #[strum(serialize = "all")]
    #[serde(rename = "all")]
    All,
    #[strum(serialize = "rcs_dlr")]
    #[serde(rename = "rcs_dlr")]
    RcsStatus,
    #[strum(serialize = "rcs_mo")]
    #[serde(rename = "rcs_mo")]
    RcsInbound,
    #[strum(serialize = "sms_mo")]
    #[serde(rename = "sms_mo")]
    SmsInbound,
    #[strum(serialize = "dlr")]
    #[serde(rename = "dlr")]
    SmsStatus,
    #[strum(serialize = "tracking")]
    #[serde(rename = "tracking")]
    Tracking,
    #[strum(serialize = "voice_call")]
    #[serde(rename = "voice_call")]
    VoiceCall,
    #[strum(serialize = "voice_status")]
    #[serde(rename = "voice_status")]
    VoiceStatus,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::All => "all",
            EventType::RcsStatus => "rcs_dlr",
            EventType::RcsInbound => "rcs_mo",
            EventType::SmsInbound => "sms_mo",
            EventType::SmsStatus => "dlr",
            EventType::Tracking => "tracking",
            EventType::VoiceCall => "voice_call",
            EventType::VoiceStatus => "voice_status",
        }
    }

    pub fn is_valid(str: &str) -> bool {
        for event_type in EventType::iter() {
            if event_type.as_str() == str {
                return true
            }
        }

        return false
    }
}

#[derive(Debug, Deserialize, EnumIter, strum_macros::Display)]
pub enum HookRequestMethod {
    GET,
    JSON,
    POST,
}

impl HookRequestMethod {
    pub fn is_valid(str: &str) -> bool {
        for method in HookRequestMethod::iter() {
            if method.to_string() == str {
                return true
            }
        }

        false
    }
}

impl Default for HookRequestMethod {
    fn default() -> Self {
        HookRequestMethod::POST
    }
}

impl Default for EventType {
    fn default() -> Self {
        panic!("Event type must be set");
    }
}

#[derive(Debug, Deserialize)]
pub struct Hook {
    pub created: String,
    pub event_filter: Option<String>,
    pub event_type: EventType,
    pub id: String,
    pub request_method: HookRequestMethod,
    pub target_url: String,
}

#[derive(Debug, Deserialize)]
pub struct HooksList {
    pub hooks: Vec<Hook>,
    pub success: bool,
}

#[derive(Default)]
pub struct HookSubscribeParams {
    pub event_filter: Option<String>,
    pub event_type: EventType,
    pub request_method: Option<HookRequestMethod>,
    pub target_url: String,
}

#[derive(Deserialize)]
pub struct HookSubscribeResponse {
    pub id: Option<u32>,
    pub success: bool,
}

pub struct HookUnsubscribeParams {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct HookUnsubscribeResponse {
    pub error_message: Option<String>,
    pub success: bool,
}

pub struct Hooks {
    client: Client
}

impl Hooks {
    pub fn new(client: Client) -> Self {
        Hooks {
            client,
        }
    }

    pub fn list(&self) -> Result<HooksList, Error> {
        let result = self.client.get(ENDPOINT)
            .call()?
            .into_json()?;
        Ok(result)
    }

    pub fn subscribe(&self, params: HookSubscribeParams) -> Result<HookSubscribeResponse, Error> {
        let result = self.client.post(ENDPOINT)
            .send_form(&[
                ("event_filter", &*params.event_filter.unwrap_or_default()),
                ("event_type", params.event_type.as_str()),
                ("request_method", &*params.request_method.unwrap_or_default().to_string()),
                ("target_url", &*params.target_url),
            ])?
            .into_json()?;
        Ok(result)
    }

    pub fn unsubscribe(&self, params: HookUnsubscribeParams) -> Result<HookUnsubscribeResponse, Error> {
        let result = self.client.delete(ENDPOINT)
            .query("id", &*params.id.to_string())
            .call()?
            .into_json()?
;
        Ok(result)
    }
}
