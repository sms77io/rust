use testutil::*;
use seven_client::rcs::{Rcs, RcsDeleteParams, RcsDispatchParams, RcsEventParams, RcsEvent};

mod testutil;

fn init() -> Rcs {
    Rcs::new(get_client())
}

#[test]
fn text() {
    let client = init();
    let result = client.dispatch(RcsDispatchParams {
        delay: Some("2050-12-31".to_string()),
        foreign_id: None,
        from: None,
        label: None,
        performance_tracking: None,
        text: "HI2U!".to_string(),
        to: "4915237035388".to_string(),
        ttl: None,
    });
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.messages.len(), 1);

    let msg = response.messages.first().unwrap();
    let id = <Option<String> as Clone>::clone(&msg.id).unwrap();

    assert!(client.delete(RcsDeleteParams {
        id,
    }).is_ok());
}

#[test]
fn delete() {
    let params = RcsDispatchParams {
        delay: Some("2026-12-12 14:02".to_string()),
        foreign_id: None,
        from: None,
        label: None,
        text: "HI2U!".to_string(),
        to: "4915237035388".to_string(),
        ttl: None,
        performance_tracking: None,
    };
    let client = init();
    let result = client.dispatch(params);
    let binding = result.unwrap();
    let message = binding.messages.first();
    let id = message.unwrap().id.clone().unwrap();

    let client = init();
    let result = client.delete(RcsDeleteParams {
        id
    });
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.success, true);
}

#[test]
fn event() {
    let params = RcsEventParams {
        event: RcsEvent::IsTyping,
        msg_id: None,
        to: "4915237035388".to_string(),
    };
    let client = init();
    let result = client.event(params);
    let response = result.unwrap();

    assert_eq!(response.success, true);
}
