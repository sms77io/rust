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
    let result = init_client().group_by_country(default_params());
    assert!(result.is_ok());

    let response = result.unwrap();
    for entry in response {
        assert_eq!(entry.country.is_empty(), false);
    }

}

#[test]
fn grouped_by_date() {
    let result = init_client().group_by_date(default_params());
    assert!(result.is_ok());

    let response = result.unwrap();
    for entry in response {
        assert_eq!(entry.date.is_empty(), false);
    }
}

#[test]
fn grouped_by_label() {
    let result = init_client().group_by_label(default_params());
    assert!(result.is_ok());
}

#[test]
fn grouped_by_subaccount() {
    let result = init_client().group_by_subaccount(default_params());
    assert!(result.is_ok());

    let response = result.unwrap();
    for entry in response {
        assert_eq!(entry.account.is_empty(), false);
    }
}
