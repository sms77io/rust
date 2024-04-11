use serde::{Deserialize};
use ureq::{Error};
use crate::client::Client;

pub struct StatusParams {
    pub msg_ids: Vec<String>
}

pub struct Status {
    client: Client
}

#[derive(Deserialize)]
pub struct StatusResponse {
    pub id: String,
    pub status: Option<String>,
    pub status_time: Option<String>,
}

impl Status {
    pub fn new(client: Client) -> Self {
        Status {
            client,
        }
    }

    pub fn get(&self, params: StatusParams) -> Result<Vec<StatusResponse>, Error> {
        Ok(self.client.get("status")
            .query("msg_id", &*params.msg_ids.join(","))
            .call()
            .unwrap()
            .into_json()?)
    }
}