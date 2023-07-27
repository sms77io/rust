use crate::{client::Client};
use ureq::{Error};
use serde::{Deserialize};

const ENDPOINT: &str = "subaccounts";

#[derive(Deserialize)]
pub struct AutoTopUp {
    pub amount: Option<f32>,
    pub threshold: Option<f32>,
}

#[derive(Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct CreateParams {
    pub email: String,
    pub name: String,
}

pub struct DeleteParams {
    pub id: u32,
}

pub struct TransferCreditsParams {
    pub id: u32,
    pub amount: f32,
}

pub struct AutoChargeParams {
    pub id: u32,
    pub amount: f32,
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

    pub fn read(&self) -> Result<Vec<Subaccount>, Error> {
        let res = self.client.request("GET", ENDPOINT)
            .clone()
            .query("action", "read")
            .call()?
            .into_json::<Vec<Subaccount>>()?;
        Ok(res)
    }

    pub fn create(&self, params: CreateParams) -> Result<CreateResponse, Error> {
        let res = self.client.request("POST", ENDPOINT)
            .send_form(&[
                ("action", "create"),
                ("email", &*params.email),
                ("name", &*params.name),
            ])?
            .into_json::<CreateResponse>()?;
        Ok(res)
    }

    pub fn transfer_credits(&self, params: TransferCreditsParams) -> Result<TransferCreditsResponse, Error> {
        let res = self.client.request("POST", ENDPOINT)
            .send_form(&[
                ("action", "transfer_credits"),
                ("amount", &params.amount.to_string()),
                ("id", &*params.id.to_string()),
            ])?
            .into_json::<TransferCreditsResponse>()?;
        Ok(res)
    }

    pub fn auto_charge(&self, params: AutoChargeParams) -> Result<AutoChargeResponse, Error> {
        let res = self.client.request("POST", ENDPOINT)
            .send_form(&[
                ("action", "update"),
                ("amount", &*params.amount.to_string()),
                ("id", &*params.id.to_string()),
                ("threshold", &*params.threshold.to_string()),
            ])?
            .into_json::<AutoChargeResponse>()?;
        Ok(res)
    }

    pub fn delete(&self, params: DeleteParams) -> Result<DeleteResponse, Error> {
        let res = self.client.request("POST", ENDPOINT)
            .send_form(&[
                ("action", "delete"),
                ("id", &*params.id.to_string()),
            ])?
            .into_json::<DeleteResponse>()?;
        Ok(res)
    }
}
