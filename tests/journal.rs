use testutil::*;
use seven_client::analytics::{AnalyticsParams, Analytics};
use seven_client::journal::{Journal, JournalParams};

mod testutil;

fn init_client() -> Journal {
    Journal::new(get_client())
}

fn default_params() -> JournalParams {
    JournalParams {
        date_from: None,
        date_to: None,
        id: None,
        state: None,
        to: None,
    }
}

#[test]
fn inbound() {
    assert!(init_client().inbound(default_params()).is_ok());
}

#[test]
fn outbound() {
    assert!(init_client().outbound(default_params()).is_ok())
}

#[test]
fn replies() {
    assert!(init_client().replies(default_params()).is_ok())
}

#[test]
fn voice() {
    assert!(init_client().voice(default_params()).is_ok())
}
