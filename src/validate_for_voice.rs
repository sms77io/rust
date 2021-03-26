use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize};

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

#[derive(Default)]
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
        Ok(self.client.request("POST", "validate_for_voice")
            .send_form(&[
                ("callback", &*params.callback.unwrap_or_default()),
                ("number", &*params.number),
            ])?
            .into_json::<ValidateForVoiceResponse>()?
        )
    }
}