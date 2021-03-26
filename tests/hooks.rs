use testutil::*;
use sms77_client::hooks::{Hooks, HookSubscribeParams, HookUnsubscribeParams};
use std::collections::HashMap;

mod testutil;

fn client() -> Hooks {
    Hooks::new(get_client())
}

#[test]
fn read() {
    assert!(client().read().is_ok());
}

#[test]
fn subscribe() {
    assert!(client().subscribe(HookSubscribeParams {
        event_type: "voice_status".to_string(),
        request_method: None,
        target_url: format!("https://rust.tld/{}", rand_str()),
    }).is_ok());
}

#[test]
fn unsubscribe() {
    assert!(client().unsubscribe(HookUnsubscribeParams { id: 840 }).is_ok());
}