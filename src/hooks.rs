use crate::client::Client;
use ureq::{Error, Request};
use serde::{Deserialize};

const ENDPOINT: &str = "hooks";

pub enum EventType {
    All,
    DLR,
    InboundSMS,
    Tracking,
    VoiceCall,
    VoiceStatus,
}

impl Default for EventType {
    fn default() -> Self {
        panic!("Event type must be set");
    }
}

#[derive(Deserialize)]
pub struct Hook {
    pub created: String,
    pub event_type: String,
    pub id: String,
    pub request_method: String,
    pub target_url: String,
}

#[derive(Deserialize)]
pub struct HooksRead {
    pub success: bool,
    pub hooks: Vec<Hook>,
}

#[derive(Default)]
pub struct HookSubscribeParams {
    pub event_filter: Option<String>,
    pub event_type: EventType,
    pub request_method: Option<String>,
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

    fn request(&self, method: &str, action: &str) -> Request {
        self.client.request(method, ENDPOINT).query("action", action)
    }

    pub fn read(&self) -> Result<HooksRead, Error> {
        Ok(self.request("GET", "read").call()?.into_json::<HooksRead>()?)
    }

    pub fn subscribe(&self, params: HookSubscribeParams) -> Result<HookSubscribeResponse, Error> {
        let event_type = match params.event_type {
            EventType::All => {"all"}
            EventType::DLR => {"dlr"}
            EventType::InboundSMS => {"sms_mo"}
            EventType::Tracking => {"tracking"}
            EventType::VoiceCall => {"voice_call"}
            EventType::VoiceStatus => {"voice_status"}
        };
        let res = self.request("POST", "subscribe")
            .send_form(&[
                ("event_type", event_type),
                ("request_method", &*params.request_method.unwrap_or_default()),
                ("target_url", &*params.target_url),
            ])?
            .into_json::<HookSubscribeResponse>()?;
        Ok(res)
    }

    pub fn unsubscribe(&self, params: HookUnsubscribeParams) -> Result<HookUnsubscribeResponse, Error> {
        Ok(self.request("POST", "unsubscribe")
            .send_form(&[("id", &*params.id.to_string())])?
            .into_json::<HookUnsubscribeResponse>()?
        )
    }
}
