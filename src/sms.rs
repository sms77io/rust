use crate::client::Client;
use ureq::{Error};
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use base64::{Engine as _, engine::{general_purpose}};

#[derive(Clone, Serialize)]
pub struct SmsParams {
    pub delay: Option<String>,
    pub files: Option<Vec<SmsFile>>,
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

#[derive(Clone)]
pub struct SmsFile {
    pub contents: String,
    pub name: String,
    pub password: Option<String>,
    pub validity: Option<u16>,
}
impl SmsFile {
    pub fn new(mut file: SmsFile) -> Self {
        let contents = file.clone().contents;
        if !general_purpose::STANDARD.decode(contents).is_ok() {
            file.contents = general_purpose::STANDARD.encode(file.contents);
        }

        file
    }
}

impl Serialize for SmsFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("SmsFile", 4)?;
        state.serialize_field("contents", &self.contents)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("password", &self.clone().password.unwrap_or_default())?;
        state.serialize_field("validity", &self.validity.unwrap_or_default())?;
        state.end()
    }
}

#[derive(Deserialize, Debug)]
pub struct SmsResponse {
    pub balance: f64,
    pub debug: String,
    pub messages: Vec<SmsMessage>,
    pub sms_type: String,
    pub success: String,
    pub total_price: f64,
}

#[derive(Clone, Deserialize, Debug)]
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
    client: Client,
}

#[derive(Serialize)]
pub struct DeleteSmsParams {
    pub ids: Vec<String>
}

#[derive(Deserialize)]
pub struct DeleteSmsResponse {
    pub deleted: Option<Vec<String>>,
    pub success: bool,
}

impl Sms {
    pub fn new(client: Client) -> Self {
        Sms {
            client,
        }
    }

    pub fn delete(&self, params: DeleteSmsParams) -> Result<DeleteSmsResponse, Error> {
        let result = self.client.delete("sms")
            .send_json(params)
            .unwrap()
            .into_json()?;

        Ok(result)
    }

    pub fn dispatch(&self, params: SmsParams) -> Result<SmsResponse, Error> {
        let result = self.client.post("sms")
            .send_json(params)
            .unwrap()
            .into_json()?;

        Ok(result)
    }
}