use testutil::*;
use seven_client::hooks::{Hooks, HookSubscribeParams, HookUnsubscribeParams};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

mod testutil;

fn rand_str() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    rand_string
}

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
