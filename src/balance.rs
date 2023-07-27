use ureq::{Error};
use crate::client::Client;

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
}
