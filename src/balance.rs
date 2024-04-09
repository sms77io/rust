use ureq::{Error};
use crate::client::Client;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct BalanceResponse {
    pub amount: f32,
    pub currency: String,
}

pub struct Balance {
    client: Client
}

impl Balance {
    pub fn new(client: Client) -> Self {
        Balance {
            client,
        }
    }

    pub fn get(&self) -> Result<BalanceResponse, Error> {
        let res = self.client.request("GET", "balance")
            .set("Accept", "application/json")
            .call()?
            .into_json()?;
        Ok(res)
    }
}
