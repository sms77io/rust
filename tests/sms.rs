use testutil::*;
use seven_client::sms::{Sms, SmsTextParams, SmsJsonParams};

mod testutil;

fn init_client() -> Sms {
    Sms::new(get_client())
}

#[test]
fn text() {
    assert!(init_client().text(SmsTextParams {
        debug: None,
        delay: None,
        details: None,
        flash: None,
        foreign_id: None,
        from: None,
        label: None,
        no_reload: None,
        performance_tracking: None,
        return_msg_id: None,
        text: "HI2U!".to_string(),
        to: "+491716992343".to_string(),
        ttl: None,
        udh: None,
        unicode: None,
        utf8: None,
    }).is_ok());
}

#[test]
fn json() {
    assert!(init_client().json(SmsJsonParams {
        debug: None,
        delay: None,
        flash: None,
        foreign_id: None,
        from: None,
        label: None,
        no_reload: None,
        performance_tracking: None,
        text: "HI2U!".to_string(),
        to: "+491716992343".to_string(),
        ttl: None,
        udh: None,
        unicode: None,
        utf8: None,
    }).is_ok());
}
