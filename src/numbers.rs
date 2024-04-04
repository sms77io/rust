use serde::{Deserialize, Serialize};
use ureq::Error;
use crate::client::Client;

#[derive(Deserialize, Serialize)]
pub struct PhoneNumber {
    pub billing: PhoneNumberBilling,
    pub country: String,
    pub created: String,
    pub expires: Option<String>,
    pub features: PhoneNumberFeatures,
    pub forward_sms_mo: PhoneNumberForwardSmsMo,
    pub friendly_name: String,
    pub number: String,
}

#[derive(Deserialize)]
pub struct ActiveNumbers {
    #[serde(rename = "activeNumbers")]
    pub active_numbers: Vec<PhoneNumber>
}

#[derive(Deserialize, Serialize)]
pub struct PhoneNumberForwardSmsMo {
    pub email: PhoneNumberForwardSmsMoEmail,
    pub sms: PhoneNumberForwardSmsMoSms,
}

#[derive(Deserialize, Serialize)]
pub struct PhoneNumberForwardSmsMoEmail {
    pub enabled: bool,
    pub address: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PhoneNumberForwardSmsMoSms {
    pub enabled: bool,
    pub number: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PhoneNumberBilling {
    pub fees: PhoneNumberBillingFees,
    pub payment_interval: String,
}

#[derive(Deserialize, Serialize)]
pub struct PhoneNumberBillingFees {
    pub basic_charge: f64,
    pub setup: f64,
    pub sms_mo: f64,
    pub voice_mo: f64,
}

#[derive(Deserialize)]
pub struct PhoneNumberOffer {
    pub country: String,
    pub features: PhoneNumberFeatures,
    pub fees: PhoneNumberOfferFees,
    pub number: String,
    pub number_parsed: String,
}

#[derive(Deserialize)]
pub struct PhoneNumberOfferFeesValues {
    pub basic_charge: f64,
    pub setup: f64,
}

#[derive(Deserialize)]
pub struct PhoneNumberOfferFees {
    pub annually: PhoneNumberOfferFeesValues,
    pub monthly: PhoneNumberOfferFeesValues,
    pub sms_mo: f64,
    pub voice_mo: f64,
}

#[derive(Deserialize, Serialize)]
pub struct PhoneNumberFeatures {
    pub a2p_sms: bool,
    pub sms: bool,
    pub voice: bool,
}

#[derive(Deserialize)]
pub struct AvailableNumbers {
    #[serde(rename = "availableNumbers")]
    pub available_numbers: Vec<PhoneNumberOffer>
}

#[derive(Serialize)]
pub struct OrderNumberParams {
    pub number: String,
    pub payment_interval: Option<PaymentInterval>,
}

#[derive(Deserialize)]
pub struct OrderNumberResponse {
    pub error: Option<String>,
    pub success: bool,
}

#[derive(Serialize)]
pub struct AvailableNumbersParams {
    pub country: Option<String>,
    pub features_a2p_sms: Option<bool>,
    pub features_sms: Option<bool>,
    pub features_voice: Option<bool>,
}

#[derive(Serialize)]
pub struct DeleteNumberParams {
    pub delete_immediately: Option<bool>,
    pub number: String,
}

#[derive(Deserialize)]
pub struct DeleteNumberResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct UpdateNumberParams {
    pub email_forward: Option<String>,
    pub friendly_name: Option<String>,
    pub number: String,
    pub sms_forward: Option<String>,
}

pub struct Numbers {
    client: Client
}

#[derive(Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentInterval {
    #[default]
    Annually,
    Monthly
}

impl PaymentInterval {
    fn as_str(&self) -> &'static str {
        match self {
            PaymentInterval::Annually => "annually",
            PaymentInterval::Monthly => "monthly"
        }
    }
}

impl Numbers {
    pub fn new(client: Client) -> Self {
        Numbers {
            client,
        }
    }

    pub fn active(&self) -> Result<ActiveNumbers, Error> {
        let res = self.client.request("GET", "numbers/active")
            .call()?
            .into_json::<ActiveNumbers>()?;
        Ok(res)
    }

    pub fn available(&self, params: AvailableNumbersParams) -> Result<AvailableNumbers, Error> {
        let res = self.client.request("GET", "numbers/available")
            .query("country", &*params.country.unwrap_or_default())
            .query("features_a2p_sms", &*bool::to_string(&params.features_a2p_sms.unwrap_or_default()))
            .query("features_sms", &*bool::to_string(&params.features_sms.unwrap_or_default()))
            .query("features_voice", &*bool::to_string(&params.features_voice.unwrap_or_default()))
            .call()?
            .into_json::<AvailableNumbers>()?;
        Ok(res)
    }

    pub fn delete(&self, params: DeleteNumberParams) -> Result<DeleteNumberResponse, Error> {
        let number = params.number;
        let endpoint = format!("numbers/active/{number}");
        let res = self.client.request("DELETE", &*endpoint)
            .query("delete_immediately", &*bool::to_string(&params.delete_immediately.unwrap_or_default()))
            .call()?
            .into_json::<DeleteNumberResponse>()?;
        Ok(res)
    }

    pub fn get(&self, number: String) -> Result<PhoneNumber, Error> {
        let endpoint = format!("numbers/active/{number}");
        let res = self.client.request("GET", &*endpoint)
            .call()?
            .into_json::<PhoneNumber>()?;
        Ok(res)
    }

    pub fn order(&self, params: OrderNumberParams) -> Result<OrderNumberResponse, Error> {
        let res = self.client.request("POST", "numbers/order")
            .send_form(&[
                ("number", &*params.number),
                ("payment_interval", params.payment_interval.unwrap_or_default().as_str()),
            ])?
            .into_json::<OrderNumberResponse>()?;
        Ok(res)
    }

    pub fn update(&self, params: UpdateNumberParams) -> Result<PhoneNumber, Error> {
        let number = params.number;
        let endpoint = format!("numbers/active/{number}");
        let res = self.client.request("PATCH", &*endpoint)
            .send_form(&[
                ("email_forward", &*params.email_forward.unwrap_or_default()),
                ("friendly_name", &*params.friendly_name.unwrap_or_default()),
                ("sms_forward", &*params.sms_forward.unwrap_or_default()),
            ])?
            .into_json::<PhoneNumber>()?;
        Ok(res)
    }
}