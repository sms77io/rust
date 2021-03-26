use testutil::*;
use sms77_client::analytics::{AnalyticsParams, Analytics};
use sms77_client::journal::{Journal, JournalParams};
use sms77_client::pricing::{PricingParams, Pricing};
use sms77_client::lookup::{Lookup, LookupParams};

mod testutil;

fn client() -> Lookup {
    Lookup::new(get_client())
}

fn params() -> LookupParams {
    LookupParams {
        number: "+491716992343".to_string()
    }
}

#[test]
fn cnam() {
    assert!(client().cnam(params()).is_ok());
}

#[test]
fn format() {
    assert!(client().format(params()).is_ok());
}

#[test]
fn hlr() {
    assert!(client().hlr(params()).is_ok());
}

#[test]
fn mnp_text() {
    assert!(client().mnp_text(params()).is_ok());
}

#[test]
fn mnp_json() {
    assert!(client().mnp_json(params()).is_ok());
}