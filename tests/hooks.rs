use testutil::{get_client, rand_str};
use seven_client::hooks::{EventType, HookRequestMethod, Hooks, HookSubscribeParams, HookUnsubscribeParams};

mod testutil;

fn client() -> Hooks {
    Hooks::new(get_client())
}

#[test]
fn list() {
    let client = client();

    let sub_result = client.subscribe(HookSubscribeParams {
        event_filter: None,
        event_type: EventType::VoiceStatus,
        request_method: None,
        target_url: format!("https://rust.tld/{}", rand_str(16)),
    });
    assert!(sub_result.is_ok());

    let result = client.list();
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.success);

    for hook in response.hooks {
        assert_ne!(hook.created, "");
        assert!(EventType::is_valid(hook.event_type.as_str()));
        assert_ne!(hook.id, "");
        assert!(HookRequestMethod::is_valid(&*hook.request_method.to_string()));
        assert_ne!(hook.target_url, "");
    }

    let id = sub_result.unwrap().id.unwrap();
    client.unsubscribe(HookUnsubscribeParams { id }).unwrap();
}

#[test]
fn subscribe() {
    let client = client();
    let result = client.subscribe(HookSubscribeParams {
        event_filter: None,
        event_type: EventType::VoiceStatus,
        request_method: None,
        target_url: format!("https://rust.tld/{}", rand_str(16)),
    });
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.success);
    assert!(response.id.unwrap() > 0);

    client.unsubscribe(HookUnsubscribeParams { id: response.id.unwrap() }).unwrap();
}

#[test]
fn unsubscribe() {
    let client = client();
    let result = client.subscribe(HookSubscribeParams {
        event_filter: None,
        event_type: EventType::VoiceStatus,
        request_method: None,
        target_url: format!("https://rust.tld/{}", rand_str(16)),
    });
    assert!(result.is_ok());

    let id = result.unwrap().id.unwrap();
    let deletion = client.unsubscribe(HookUnsubscribeParams { id });
    assert!(deletion.is_ok());
    let res = deletion.unwrap();
    assert!(res.success);
    assert!(Option::is_none(&res.error_message));
}
