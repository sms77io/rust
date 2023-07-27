use testutil::*;
use seven_client::analytics::{AnalyticsParams, Analytics};
use seven_client::journal::{Journal, JournalParams};
use seven_client::pricing::{PricingParams, Pricing};

mod testutil;

fn init_client() -> Pricing {
    Pricing::new(get_client())
}

fn default_params() -> PricingParams {
    PricingParams {
        country: None
    }
}

#[test]
fn csv() {
    assert!(init_client().csv(default_params()).is_ok());
}

#[test]
fn json() {
    assert!(init_client().json(default_params()).is_ok())
}
