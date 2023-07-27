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
    let res = init_client().group_by_country(default_params());
    assert!(res.is_ok());
}

#[test]
fn grouped_by_date() {
    let res = init_client().group_by_date(default_params());
    assert!(res.is_ok())
}

#[test]
fn grouped_by_label() {
    let res = init_client().group_by_label(default_params());
    assert!(res.is_ok())
}

#[test]
fn grouped_by_subaccount() {
    let res = init_client().group_by_subaccount(default_params());
    assert!(res.is_ok())
}
