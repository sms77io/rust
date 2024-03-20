use testutil::*;
use seven_client::rcs::{Rcs, RcsDeleteParams, RcsDispatchParams, RcsEventParams, RcsEvent};

mod testutil;

fn init_client() -> Rcs {
    Rcs::new(get_client())
}

#[test]
fn text() {
    let params = RcsDispatchParams {
        delay: None,
        foreign_id: None,
        from: None,
        label: None,
        performance_tracking: None,
        text: "HI2U!".to_string(),
        to: "4915237035388".to_string(),
        ttl: None,
    };
    let client = init_client();
    let result = client.dispatch(params);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.messages.len(), 1);
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
    let client = init_client();
    let result = client.dispatch(params);
    let binding = result.unwrap();
    let message = binding.messages.first();
    let id = message.unwrap().id.clone().unwrap().parse::<u64>().unwrap();

    let params = RcsDeleteParams {
        id
    };
    let client = init_client();
    let result = client.delete(params);
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
    let client = init_client();
    let result = client.event(params);
    let response = result.unwrap();

    assert_eq!(response.success, true);
}
