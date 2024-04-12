use crate::client::Client;
use ureq::{Error, Request};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CountryNetwork {
    pub comment: Option<String>,
    pub features: Vec<String>,
    pub mcc: String,
    pub mncs: Option<Vec<String>>,
    #[serde(rename = "networkName")]
    pub network_name: Option<String>,
    pub price: f64,
}

#[derive(Deserialize)]
pub struct CountryPricing {
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "countryName")]
    pub country_name: String,
    #[serde(rename = "countryPrefix")]
    pub country_prefix: String,
    pub networks: Vec<CountryNetwork>,
}

#[derive(Deserialize)]
pub struct PricingResponse {
    #[serde(rename = "countCountries")]
    pub count_countries: u16,
    #[serde(rename = "countNetworks")]
    pub count_networks: u32,
    pub countries: Vec<CountryPricing>,
}

pub struct PricingParams {
    pub country: Option<String>,
}

pub struct Pricing {
    client: Client
}

impl Pricing {
    pub fn new(client: Client) -> Self {
        Pricing {
            client,
        }
    }

    pub fn csv(&self, params: PricingParams) -> Result<String, Error> {
        Ok(self.get(params, "csv").call()?.into_string()?)
    }

    pub fn json(&self, params: PricingParams) -> Result<PricingResponse, Error> {
        Ok(self.get(params, "json").call()?.into_json::<PricingResponse>()?)
    }

    fn get(&self, params: PricingParams, format: &str) -> Request {
        let mut req = self.client.get( "pricing").clone();

        if params.country.is_some() {
            req = req.query("country", &*params.country.unwrap_or_default().to_string());
        }

        req.query("format", format)
    }
}