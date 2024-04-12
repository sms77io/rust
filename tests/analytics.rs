extern crate chrono;

use std::ops::Sub;
use chrono::{Local, TimeZone, Duration};
use testutil::*;
use seven_client::analytics::{AnalyticsParams, Analytics};

mod testutil;

fn init() -> Analytics {
    Analytics::new(get_client())
}

#[test]
fn grouped_by_country() {
    let format = "%Y-%m-%d";
    let now = Local::now();
    let end = now.format(format).to_string();
    let start = (now - Duration::days(30)).format(format).to_string();
    let result = init().group_by_country(AnalyticsParams {
        end: Some(end),
        label: None,
        start: Some(start),
        subaccounts: None,
    });
    assert!(result.is_ok());

    for entry in result.unwrap() {
        assert!(entry.usage_eur >= 0.0);
    }
}

#[test]
fn grouped_by_date() {
    let result = init().group_by_date(AnalyticsParams {
        end: None,
        label: None,
        start: None,
        subaccounts: None,
    });
    assert!(result.is_ok());

    for entry in result.unwrap() {
        assert!(!entry.date.is_empty());
        assert!(entry.usage_eur >= 0.0);
    }
}

#[test]
fn grouped_by_label() {
    let result = init().group_by_label(AnalyticsParams {
        end: None,
        label: None,
        start: None,
        subaccounts: None,
    });
    assert!(result.is_ok());

    for entry in result.unwrap() {
        assert!(entry.usage_eur >= 0.0);
    }
}

#[test]
fn grouped_by_subaccount() {
    let result = init().group_by_subaccount(AnalyticsParams {
        end: None,
        label: None,
        start: None,
        subaccounts: None,
    });
    assert!(result.is_ok());

    for entry in result.unwrap() {
        assert!(!entry.account.is_empty());
    }
}