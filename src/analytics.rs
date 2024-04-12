use crate::{client::Client};
use ureq::{Error, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize)]
pub struct AnalyticsParams {
    pub end: Option<String>,
    pub label: Option<String>,
    pub start: Option<String>,
    pub subaccounts: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticBase {
    pub hlr: u32,
    pub inbound: u32,
    pub mnp: u32,
    pub sms: u32,
    pub usage_eur: f64,
    pub voice: u32,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticByCountry {
    #[serde(flatten)]
    base: AnalyticBase,
    pub country: String,
}

impl std::ops::Deref for AnalyticByCountry {
    type Target = AnalyticBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[derive(Debug, Deserialize)]
pub struct AnalyticByDate {
    #[serde(flatten)]
    base: AnalyticBase,
    pub date: String,
}
impl std::ops::Deref for AnalyticByDate {
    type Target = AnalyticBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
#[derive(Debug, Deserialize)]
pub struct AnalyticByLabel {
    #[serde(flatten)]
    base: AnalyticBase,
    pub label: String,
}
impl std::ops::Deref for AnalyticByLabel {
    type Target = AnalyticBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
#[derive(Debug, Deserialize)]
pub struct AnalyticBySubaccount {
    pub account: String,
    #[serde(flatten)]
    base: AnalyticBase,
}
impl std::ops::Deref for AnalyticBySubaccount {
    type Target = AnalyticBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
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

    pub fn group_by_country(&self, params: AnalyticsParams) -> Result<Vec<AnalyticByCountry>, Error> {
        let res = self.get(params, "country")
            .call()?
            .into_json()?;
        Ok(res)
    }

    pub fn group_by_date(&self, params: AnalyticsParams) -> Result<Vec<AnalyticByDate>, Error> {
        let res = self.get(params, "date")
            .call()?
            .into_json()?;
        Ok(res)
    }

    pub fn group_by_label(&self, params: AnalyticsParams) -> Result<Vec<AnalyticByLabel>, Error> {
        let res = self.get(params, "label")
            .call()?
            .into_json()?;
        Ok(res)
    }

    pub fn group_by_subaccount(&self, params: AnalyticsParams) -> Result<Vec<AnalyticBySubaccount>, Error> {
        let res = self.get(params, "subaccount")
            .call()?
            .into_json()?;
        Ok(res)
    }

    fn get(&self, params: AnalyticsParams, group_by: &str) -> Request {
        let qs = serde_qs::to_string(&params).unwrap();
        let path = format!("analytics?{}", qs);
        self.client.get(&*path).query("group_by", group_by)
    }
}
