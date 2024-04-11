use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ValidateForVoiceResponse {
    pub code: Option<String>,
    pub error: Option<String>,
    pub formatted_output: Option<String>,
    pub id: Option<u64>,
    pub sender: Option<String>,
    pub success: bool,
    pub voice: Option<bool>,
}

#[derive(Default, Serialize)]
pub struct ValidateForVoiceParams {
    pub callback: Option<String>,
    pub number: String,
}

pub struct ValidateForVoice {
    client: Client
}

impl ValidateForVoice {
    pub fn new(client: Client) -> Self {
        ValidateForVoice {
            client,
        }
    }

    pub fn post(&self, params: ValidateForVoiceParams) -> Result<ValidateForVoiceResponse, Error> {
        Ok(self.client.post("validate_for_voice")
            .send_json(params)?
            .into_json()?
        )
    }
}