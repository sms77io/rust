use testutil::*;
use seven_client::journal::{Journal, JournalParams};

mod testutil;

fn init_client() -> Journal {
    Journal::new(get_client())
}

const DEFAULT_PARAMS: JournalParams = JournalParams {
    date_from: None,
    date_to: None,
    id: None,
    limit: None,
    offset: None,
    state: None,
    to: None,
};

#[test]
fn inbound() {
    assert!(init_client().inbound(DEFAULT_PARAMS).is_ok());
}

#[test]
fn outbound() {
    assert!(init_client().outbound(DEFAULT_PARAMS).is_ok())
}

#[test]
fn replies() {
    assert!(init_client().replies(DEFAULT_PARAMS).is_ok())
}

#[test]
fn voice() {
    assert!(init_client().voice(DEFAULT_PARAMS).is_ok())
}
