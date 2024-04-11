use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct VoiceResponse {
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

#[derive(Serialize)]
pub struct VoiceParams {
    pub from: Option<String>,
    pub ringtime: Option<u8>,
    pub text: String,
    pub to: String,
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

    pub fn dispatch(&self, params: VoiceParams) -> Result<VoiceResponse, Error> {
        Ok(self.client.post("voice")
            .send_json(params)?
            .into_json()?)
    }
}