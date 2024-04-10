use testutil::*;
use seven_client::pricing::{PricingParams, Pricing};

mod testutil;

fn init_client() -> Pricing {
    Pricing::new(get_client())
}

const DEFAULT_PARAMS: PricingParams =  PricingParams {
    country: None
};

#[test]
fn csv() {
    assert!(init_client().csv(DEFAULT_PARAMS).is_ok());
}

#[test]
fn json() {
    assert!(init_client().json(DEFAULT_PARAMS).is_ok())
}

#[test]
fn json_country() {
    let country = Some("FR".to_string());
    let result = init_client().json(PricingParams{country: country.clone()});
    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.countries.len(), 1);
    assert_eq!(response.count_countries, response.countries.len() as u16);

    let country_pricing = response.countries.first().unwrap();
    assert_eq!(country_pricing.country_code, country.unwrap());

    assert_eq!(response.count_networks, country_pricing.networks.len() as u32);
}