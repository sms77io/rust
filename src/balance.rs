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

    pub fn get(&self) -> Result<f64, Error> {
        let res = self.client.request("GET", "balance")
            .call()?
            .into_json::<f64>()?;
        Ok(res)
    }

    pub fn json(&self) -> Result<BalanceResponse, Error> {
        let res = self.client.request("GET", "balance")
            .set("Accept", "application/json")
            .call()?
            .into_json::<BalanceResponse>()?;
        Ok(res)
    }
}
