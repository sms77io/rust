use crate::client::Client;
use crate::to_string;
use ureq::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

fn to_roaming<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Roaming, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => Roaming {
            roaming_country_code: "".to_string(),
            roaming_network_code: "".to_string(),
            roaming_network_name: "".to_string(),
            status: s,
        },
        _ => return Err(de::Error::custom("wrong type"))
    })
}

#[derive(Deserialize)]
pub struct CallingNameDelivery {
    pub code: String,
    pub name: String,
    pub number: String,
    pub success: String,
}

#[derive(Deserialize)]
pub struct Carrier {
    pub country: String,
    pub name: String,
    pub network_code: String,
    pub network_type: String,
}

#[derive(Deserialize)]
pub struct NumberFormat {
    pub national: String,
    pub carrier: String,
    pub country_code: String,
    pub country_iso: String,
    pub country_name: String,
    pub international: String,
    pub international_formatted: String,
    pub network_type: String,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct HomeLocationRegister {
    pub country_code: String,
    pub country_code_iso3: Option<String>,
    pub country_name: String,
    pub country_prefix: String,
    pub current_carrier: Carrier,
    pub gsm_code: String,
    pub gsm_message: String,
    pub international_format_number: String,
    pub international_formatted: String,
    #[serde(deserialize_with = "to_string")]
    pub lookup_outcome: String,
    pub lookup_outcome_message: String,
    pub national_format_number: String,
    pub original_carrier: Carrier,
    pub ported: String,
    pub reachable: String,
    #[serde(deserialize_with = "to_roaming")]
    pub roaming: Roaming,
    pub status: bool,
    pub status_message: String,
    pub valid_number: String,
}

#[derive(Deserialize)]
pub struct Mnp {
    pub country: String,
    pub international_formatted: String,
    #[serde(rename = "isPorted")]
    pub is_ported: bool,
    pub mccmnc: String,
    pub national_format: String,
    pub network: String,
    pub number: String,
}

#[derive(Deserialize)]
pub struct MobileNumberPortability {
    pub code: u16,
    pub mnp: Mnp,
    pub price: f64,
    pub success: bool,
}

pub struct Roaming {
    pub roaming_country_code: String,
    pub roaming_network_code: String,
    pub roaming_network_name: String,
    pub status: String,
}

pub struct LookupParams {
    pub number: String,
}

pub struct Lookup {
    client: Client
}

impl Lookup {
    pub fn new(client: Client) -> Self {
        Lookup {
            client,
        }
    }

    fn post(&self, params: LookupParams, type_: &str, json: bool) -> Result<Response, Error> {
        let req = self.client.request("POST", "lookup").clone();

        Ok(req.send_form(&[
            ("json", self.client.bool_to_string(json)),
            ("number", &*params.number),
            ("type", type_),
        ])?)
    }

    pub fn cnam(&self, params: LookupParams) -> Result<CallingNameDelivery, Error> {
        Ok(self.post(params, "cnam", false).unwrap().into_json::<CallingNameDelivery>()?)
    }

    pub fn format(&self, params: LookupParams) -> Result<NumberFormat, Error> {
        Ok(self.post(params, "format", false).unwrap().into_json::<NumberFormat>()?)
    }

    pub fn hlr(&self, params: LookupParams) -> Result<HomeLocationRegister, Error> {
        Ok(self.post(params, "hlr", false).unwrap().into_json::<HomeLocationRegister>()?)
    }

    pub fn mnp_text(&self, params: LookupParams) -> Result<String, Error> {
        Ok(self.post(params, "mnp", false).unwrap().into_string()?)
    }

    pub fn mnp_json(&self, params: LookupParams) -> Result<MobileNumberPortability, Error> {
        Ok(self.post(params, "mnp", true).unwrap().into_json::<MobileNumberPortability>()?)
    }
}