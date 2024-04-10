use testutil::*;
use seven_client::sms::{Sms, SmsParams};

mod testutil;

fn init_client() -> Sms {
    Sms::new(get_client())
}

#[test]
fn dispatch() {
    let text = "HI2U!".to_string();
    let to = "491716992343".to_string();
    let params = SmsParams {
        delay: None,
        flash: None,
        foreign_id: None,
        from: None,
        label: None,
        performance_tracking: None,
        text: text.clone(),
        to: to.clone(),
        ttl: None,
        udh: None,
    };
    let result = init_client().dispatch(params);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.messages.len(), 1);

    let msg = response.messages.first().unwrap();
    assert_eq!(msg.text, text);
    assert_eq!(msg.recipient, to)
}
