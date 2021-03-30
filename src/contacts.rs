use crate::{client::Client};
use ureq::{Error, Request};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Contact {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Number")]
    pub number: String,
}

#[derive(Default)]
pub struct ContactDeleteParams {
    pub id: u32,
}

#[derive(Default)]
pub struct ContactsReadParams {
    pub id: Option<u32>,
}

#[derive(Default)]
pub struct ContactEditParams {
    pub empfaenger: Option<String>,
    pub id: u32,
    pub nick: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContactWriteResponse {
    pub id: u64,
    #[serde(rename = "return")]
    pub return_: String,
}

#[derive(Debug, Deserialize)]
pub struct ContactEditResponse {
    #[serde(rename = "return")]
    pub return_: String,
}

pub struct Contacts {
    client: Client
}

impl Contacts {
    pub fn new(client: Client) -> Self {
        Contacts {
            client,
        }
    }

    pub fn request(&self, method: &str, action: &str) -> Request {
        let req = self.client.request(method, "contacts").clone();

        req.query("action", action)
    }

    pub fn read(&self, params: ContactsReadParams) -> Result<String, Error> {
        Ok(self.request("GET", "read")
            .query("id", &*params.id.unwrap_or_default().to_string())
            .call()?.into_string()?)
    }

    pub fn read_json(&self, params: ContactsReadParams) -> Result<Vec<Contact>, Error> {
        Ok(self.request("GET", "read")
            .query("id", &*params.id.unwrap_or_default().to_string())
            .query("json", "1")
            .call()?.into_json::<Vec<Contact>>()?)
    }

    fn _write(&self, json: bool, _params: Option<ContactEditParams>) -> Request {
        let mut req = self.request("GET", "write").clone();

        if !_params.is_none() {
            let params = _params.unwrap();

            if params.nick.is_none() {
                req = req.query("nick", &*params.nick.unwrap_or_default().to_string());
            }

            if params.empfaenger.is_none() {
                req = req.query(
                    "empfaenger", &*params.empfaenger.unwrap_or_default().to_string());
            }

            req = req.query("id", &*params.id.to_string());
        }

        if json {
            req = req.query("json", "1");
        }

        req
    }

    pub fn create(&self) -> Result<String, Error> {
        Ok(self._write(false, None).call()?.into_string()?)
    }

    pub fn create_json(&self) -> Result<ContactWriteResponse, Error> {
        Ok(self._write(true, None).call()?
            .into_json::<ContactWriteResponse>()?)
    }

    pub fn edit(&self, params: ContactEditParams) -> Result<String, Error> {
        Ok(self._write(false, Option::from(params))
            .call()?.into_string()?)
    }

    pub fn edit_json(&self, params: ContactEditParams) -> Result<ContactEditResponse, Error> {
        Ok(self._write(true, Option::from(params))
            .call()?.into_json::<ContactEditResponse>()?)
    }

    pub fn delete(&self, params: ContactDeleteParams) -> Result<u8, Error> {
        Ok(self.request("POST", "del")
            .query("id", &*params.id.to_string()).call()?.into_json::<u8>()?)
    }
}