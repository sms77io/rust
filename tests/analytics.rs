use testutil::*;
use seven_client::analytics::{AnalyticsParams, Analytics};

mod testutil;

fn init_client() -> Analytics {
    Analytics::new(get_client())
}

fn default_params() -> AnalyticsParams {
    AnalyticsParams {
        end: None,
        label: None,
        start: None,
        subaccounts: None,
    }
}

#[test]
fn grouped_by_country() {
    assert!(init_client().group_by_country(default_params()).is_ok());
}

#[test]
fn grouped_by_date() {
    assert!(init_client().group_by_date(default_params()).is_ok())
}

#[test]
fn grouped_by_label() {
    assert!(init_client().group_by_label(default_params()).is_ok())
}

#[test]
fn grouped_by_subaccount() {
    assert!(init_client().group_by_subaccount(default_params()).is_ok())
}
