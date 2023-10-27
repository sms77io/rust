use crate::client::Client;
use ureq::{Error, Response};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct VoiceJson {
    pub balance: f64,
    pub debug: bool,
    pub messages: Vec<VoiceMessage>,
    pub success: String,
    pub total_price: f64,
}

#[derive(Deserialize)]
pub struct VoiceMessage {
    pub error: Option<String>,
    pub error_text: Option<String>,
    pub id: Option<String>,
    pub price: f64,
    pub recipient: String,
    pub sender: String,
    pub success: bool,
    pub text: String,
}

pub struct VoiceParams {
    pub debug: Option<bool>,
    pub from: Option<String>,
    pub ringtime: Option<u8>,
    pub text: String,
    pub to: String,
    pub xml: Option<bool>,
}

pub struct Voice {
    client: Client
}

impl Voice {
    pub fn new(client: Client) -> Self {
        Voice {
            client,
        }
    }

    fn post(&self, params: VoiceParams, json: bool) -> Result<Response, Error> {
        Ok(self.client.request("POST", "voice")
            .send_form(&[
                ("debug", self.client.bool_to_string(params.debug.unwrap_or_default())),
                ("from", &*params.from.unwrap_or_default()),
                ("json", self.client.bool_to_string(json)),
                ("ringtime", &*params.ringtime.unwrap_or_default().to_string()),
                ("text", &*params.text),
                ("to", &*params.to),
                ("xml", self.client.bool_to_string(params.xml.unwrap_or_default())),
            ])?)
    }

    pub fn text(&self, params: VoiceParams) -> Result<String, Error> {
        Ok(self.post(params, false).unwrap().into_string()?)
    }

    pub fn json(&self, params: VoiceParams) -> Result<VoiceJson, Error> {
        Ok(self.post(params, true).unwrap().into_json::<VoiceJson>()?)
    }
}