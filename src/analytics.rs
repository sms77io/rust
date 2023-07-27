use crate::{client::Client, to_string};
use ureq::{Error, Request};
use serde::{Deserialize};

#[derive(Default)]
pub struct AnalyticsParams {
    pub end: Option<String>,
    pub label: Option<String>,
    pub start: Option<String>,
    pub subaccounts: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticByCountry {
    #[serde(deserialize_with = "to_string")]
    pub country: String,
    pub hlr: u32,
    pub inbound: u32,
    pub mnp: u32,
    pub sms: u32,
    pub usage_eur: f64,
    pub voice: u32,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticByDate {
    pub date: String,
    pub hlr: u32,
    pub inbound: u32,
    pub mnp: u32,
    pub sms: u32,
    pub usage_eur: f64,
    pub voice: u32,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticByLabel {
    pub hlr: u32,
    pub inbound: u32,
    #[serde(deserialize_with = "to_string")]
    pub label: String,
    pub mnp: u32,
    pub sms: u32,
    pub usage_eur: f64,
    pub voice: u32,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticBySubaccount {
    pub account: String,
    pub hlr: u32,
    pub inbound: u32,
    pub mnp: u32,
    pub sms: u32,
    pub usage_eur: f64,
    pub voice: u32,
}

pub struct Analytics {
    client: Client
}

impl Analytics {
    pub fn new(client: Client) -> Self {
        Analytics {
            client,
        }
    }

    pub fn get(&self, params: AnalyticsParams, group_by: &str) -> Request {
        let mut req = self.client.request("GET", "analytics").clone();

        if params.end.is_some() {
            req = req.query("end", &*params.end.unwrap_or_default());
        }
        if params.label.is_some() {
            req = req.query("label", &*params.label.unwrap_or_default());
        }
        if params.start.is_some() {
            req = req.query("start", &*params.start.unwrap_or_default());
        }
        if params.subaccounts.is_some() {
            req = req.query("subaccounts", &*params.subaccounts.unwrap_or_default());
        }

        req.query("group_by", group_by)
    }

    pub fn group_by_country(&self, params: AnalyticsParams) -> Result<Vec<AnalyticByCountry>, Error> {
        let res = self.get(params, "country").call()?.into_json::<Vec<AnalyticByCountry>>()?;
        Ok(res)
    }

    pub fn group_by_date(&self, params: AnalyticsParams) -> Result<Vec<AnalyticByDate>, Error> {
        let res = self.get(params, "date").call()?.into_json::<Vec<AnalyticByDate>>()?;
        Ok(res)
    }

    pub fn group_by_label(&self, params: AnalyticsParams) -> Result<Vec<AnalyticByLabel>, Error> {
        let res = self.get(params, "label").call()?.into_json::<Vec<AnalyticByLabel>>()?;
        Ok(res)
    }

    pub fn group_by_subaccount(&self, params: AnalyticsParams) -> Result<Vec<AnalyticBySubaccount>, Error> {
        let res = self.get(params, "subaccount").call()?.into_json::<Vec<AnalyticBySubaccount>>()?;
        Ok(res)
    }
}
