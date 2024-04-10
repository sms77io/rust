use serde::Deserialize;
use ureq::{Error, Request};
use serde_aux::prelude::*;
use crate::client::Client;

#[derive(Deserialize)]
pub struct JournalInbound {
    pub from: String,
    pub id: String,
    pub price: String,
    pub text: String,
    pub timestamp: String,
    pub to: String,
}

#[derive(Deserialize)]
pub struct JournalOutbound {
    pub connection: String,
    pub dlr: Option<String>,
    pub dlr_timestamp: Option<String>,
    pub foreign_id: Option<String>,
    pub from: String,
    pub id: String,
    pub label: Option<String>,
    pub latency: Option<String>,
    pub mccmnc: Option<String>,
    pub price: String,
    pub text: String,
    pub timestamp: String,
    pub to: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default)]
pub struct JournalParams {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub id: Option<u64>,
    pub limit: Option<u8>,
    pub offset: Option<u16>,
    pub state: Option<String>,
    pub to: Option<String>,
}

#[derive(Deserialize)]
pub struct JournalReply {
    pub from: String,
    pub id: String,
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub price: String,
    pub text: String,
    pub timestamp: String,
    pub to: String,
}

#[derive(Deserialize)]
pub struct JournalVoice {
    pub duration: Option<String>,
    pub error: Option<String>,
    pub from: String,
    pub id: String,
    pub price: Option<String>,
    pub status: String,
    pub text: String,
    pub timestamp: String,
    pub to: String,
    pub xml: bool,
}

pub struct Journal {
    client: Client
}

impl Journal {
    pub fn new(client: Client) -> Self {
        Journal {
            client,
        }
    }

    fn get(&self, params: JournalParams, type_: &str) -> Request {
        let mut req = self.client.get(&*format!("journal/{}", type_)).clone();

        if params.id.is_some() {
            req = req.query("id", &*params.id.unwrap_or_default().to_string());
        }
        if params.date_from.is_some() {
            req = req.query("date_from", &*params.date_from.unwrap_or_default());
        }
        if params.date_to.is_some() {
            req = req.query("date_to", &*params.date_to.unwrap_or_default());
        }
        if params.to.is_some() {
            req = req.query("to", &*params.to.unwrap_or_default());
        }
        if params.state.is_some() {
            req = req.query("state", &*params.state.unwrap_or_default());
        }

        req
    }

    pub fn inbound(&self, params: JournalParams) -> Result<Vec<JournalInbound>, Error> {
        Ok(self.get(params, "inbound").call()?.into_json()?)
    }

    pub fn outbound(&self, params: JournalParams) -> Result<Vec<JournalOutbound>, Error> {
        Ok(self.get(params, "outbound").call()?.into_json()?)
    }

    pub fn replies(&self, params: JournalParams) -> Result<Vec<JournalReply>, Error> {
        Ok(self.get(params, "replies").call()?.into_json()?)
    }

    pub fn voice(&self, params: JournalParams) -> Result<Vec<JournalVoice>, Error> {
        Ok(self.get(params, "voice").call()?.into_json()?)
    }
}