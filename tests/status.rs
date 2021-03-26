use testutil::*;
use sms77_client::sms::{Sms, SmsTextParams, SmsJsonParams};
use sms77_client::status::{Status, StatusParams};

mod testutil;

fn init_client() -> Status {
    Status::new(get_client())
}

#[test]
fn text() {
    assert!(init_client().text(StatusParams {
        msg_id: 77136739797,
    }).is_ok());
}