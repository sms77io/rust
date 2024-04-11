use strum::IntoEnumIterator;
use crate::client::Client;
use crate::to_string;
use ureq::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;
use strum_macros::EnumIter;

#[derive(Deserialize, EnumIter)]
pub enum RcsCapability {
    ActionCreateCalendarEvent,
    ActionDial,
    ActionOpenUrl,
    ActionShareLocation,
    ActionViewLocation,
    FeatureUnspecified,
    Revocation,
    RichcardCarousel,
    RichcardStandalone,
}

impl RcsCapability {
    pub fn as_str(&self) -> &'static str {
        match self {
            RcsCapability::ActionCreateCalendarEvent => "ACTION_CREATE_CALENDAR_EVENT",
            RcsCapability::ActionDial => "ACTION_DIAL",
            RcsCapability::ActionOpenUrl => "ACTION_OPEN_URL",
            RcsCapability::ActionShareLocation => "ACTION_SHARE_LOCATION",
            RcsCapability::ActionViewLocation => "ACTION_VIEW_LOCATION",
            RcsCapability::FeatureUnspecified => "FEATURE_UNSPECIFIED",
            RcsCapability::Revocation => "REVOCATION",
            RcsCapability::RichcardCarousel => "RICHCARD_CAROUSEL",
            RcsCapability::RichcardStandalone => "RICHCARD_STANDALONE",
        }
    }

/*    pub fn iterator() -> Iter<'static, str> {
        static CAPABILITIES: [str; 9] = [
            *RcsCapability::ActionCreateCalendarEvent.as_str(),
            *RcsCapability::ActionDial.as_str(),
            *RcsCapability::ActionOpenUrl.as_str(),
            *RcsCapability::ActionShareLocation.as_str(),
            *RcsCapability::ActionViewLocation.as_str(),
            *RcsCapability::FeatureUnspecified.as_str(),
            *RcsCapability::Revocation.as_str(),
            *RcsCapability::RichcardCarousel.as_str(),
            *RcsCapability::RichcardStandalone.as_str(),
        ];
        CAPABILITIES.iter()
    }*/

    pub fn is_valid(str: &str) -> bool {
        for capability in RcsCapability::iter() {
            if capability.as_str() == str {
                return true
            }
        }

        return false
    }
}

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
    pub carrier: String,
    pub country_code: String,
    pub country_iso: String,
    pub country_name: String,
    pub international: String,
    pub international_formatted: String,
    pub national: String,
    pub network_type: String,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct  RcsCapabilities  {
    pub carrier: String,
    pub country_code: String,
    pub country_iso: String,
    pub country_name: String,
    pub international: String,
    pub international_formatted: String,
    pub national: String,
    pub network_type: String,
    pub rcs_capabilities: Vec<RcsCapability>,
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

    pub fn cnam(&self, params: LookupParams) -> Result<CallingNameDelivery, Error> {
        let result = self.get(params, "cnam")
            .unwrap()
            .into_json()?;
        Ok(result)
    }

    pub fn format(&self, params: LookupParams) -> Result<NumberFormat, Error> {
        let result = self.get(params, "format")
            .unwrap()
            .into_json()?;
        Ok(result)
    }

    pub fn hlr(&self, params: LookupParams) -> Result<HomeLocationRegister, Error> {
        let result = self.get(params, "hlr")
            .unwrap()
            .into_json()?;
        Ok(result)
    }

    pub fn mnp(&self, params: LookupParams) -> Result<MobileNumberPortability, Error> {
        let result = self.get(params, "mnp")
            .unwrap()
            .into_json()?;
        Ok(result)
    }

    pub fn rcs(&self, params: LookupParams) -> Result<RcsCapabilities, Error> {
        let result = self.get(params, "rcs")
            .unwrap()
            .into_json()?;

        Ok(result)
    }

    fn get(&self, params: LookupParams, type_: &str) -> Result<Response, Error> {
        let result = self.client.get(format!("lookup/{}", type_).as_str())
            .query("number", &*params.number)
            .call()?;

        Ok(result)
    }
}
