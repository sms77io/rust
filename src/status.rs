use crate::client::Client;
use ureq::{Error};

pub struct StatusParams {
    pub msg_id: u64
}

pub struct Status {
    client: Client
}

impl Status {
    pub fn new(client: Client) -> Self {
        Status {
            client,
        }
    }

    pub fn text(&self, params: StatusParams) -> Result<String, Error> {
        Ok(self.client.request("GET", "status")
            .query("msg_id", &*params.msg_id.to_string())
            .call()?
            .into_string()?)
    }
}