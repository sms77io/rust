use std::io::ErrorKind;
use crate::{client::Client};
use serde::{Deserialize, Serialize};
use ureq;

const ENDPOINT: &str = "subaccounts";

#[derive(Clone, Debug, Deserialize)]
pub struct AutoTopUp {
    pub amount: Option<f32>,
    pub threshold: Option<f32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Contact {
    pub email: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Subaccount {
    pub auto_topup: AutoTopUp,
    pub balance: f32,
    pub company: Option<String>,
    pub contact: Contact,
    pub id: u32,
    pub total_usage: f32,
    pub username: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateResponse {
    pub error: Option<String>,
    pub subaccount: Option<Subaccount>,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct TransferCreditsResponse {
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct AutoChargeResponse {
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct DeleteResponse {
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Serialize)]
pub struct CreateParams {
    pub email: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct DeleteParams {
    pub id: u32,
}

#[derive(Serialize)]
pub struct TransferCreditsParams {
    pub amount: f32,
    pub id: u32,
}

#[derive(Serialize)]
pub struct AutoChargeParams {
    pub amount: f32,
    pub id: u32,
    pub threshold: f32,
}

pub struct Subaccounts {
    client: Client,
}

impl Subaccounts {
    pub fn new(client: Client) -> Self {
        Subaccounts {
            client,
        }
    }

    pub fn get(&self, id: u32) -> Result<Subaccount, ureq::Error> {
        let result = self.client.get(ENDPOINT)
            .query("action", "read")
            .query("id", &*id.to_string())
            .call()?
            .into_json::<Vec<Subaccount>>();

        if result.is_err() {
            return Err(ureq::Error::from(result.err().unwrap()));
        }

        let response = result.unwrap();
        let first = response.first();
        let has_entries = response.len() > 0;
        if has_entries {
            return Ok(first.unwrap().clone());
        }

        return Err(ureq::Error::from(std::io::Error::from(ErrorKind::NotFound)));
    }

    pub fn list(&self) -> Result<Vec<Subaccount>, ureq::Error> {
        let res = self.client.get(ENDPOINT)
            .query("action", "read")
            .call()?
            .into_json()?;
        Ok(res)
    }

    pub fn create(&self, params: CreateParams) -> Result<CreateResponse, ureq::Error> {
        let res = self.client.post(ENDPOINT)
            .query("action", "create")
            .send_json(params)?
            .into_json()?;
        Ok(res)
    }

    pub fn transfer_credits(&self, params: TransferCreditsParams) -> Result<TransferCreditsResponse, ureq::Error> {
        let res = self.client.post(ENDPOINT)
            .query("action", "transfer_credits")
            .send_json(params)?
            .into_json()?;
        Ok(res)
    }

    pub fn auto_charge(&self, params: AutoChargeParams) -> Result<AutoChargeResponse, ureq::Error> {
        let res = self.client.post(ENDPOINT)
            .query("action", "update")
            .send_json(params)?
            .into_json()?;
        Ok(res)
    }

    pub fn delete(&self, params: DeleteParams) -> Result<DeleteResponse, ureq::Error> {
        let res = self.client.post(ENDPOINT)
            .query("action", "delete")
            .send_json(params)?
            .into_json()?;
        Ok(res)
    }
}
