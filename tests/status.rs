use seven_client::sms::{DeleteSmsParams, Sms, SmsParams};
use testutil::*;
use seven_client::status::{Status, StatusParams};

mod testutil;

fn init_resource() -> Status {
    Status::new(get_client())
}

#[test]
fn empty() {
    let result = init_resource().get(StatusParams {
        msg_ids: vec![],
    });
    assert!(!result.is_ok());
}

#[test]
fn invalid() {
    let msg_ids = vec!["1".to_string()];
    let msg_count = msg_ids.len();
    let result = init_resource().get(StatusParams {
        msg_ids: msg_ids.clone(),
    });
    assert!(result.is_ok());

    let statuses = result.unwrap();
    assert_eq!(msg_count, statuses.len());

    let status = statuses.first().unwrap();
    assert!(msg_ids.contains(&status.id));
    assert_eq!(None, status.status);
    assert_eq!(None,  status.status_time);
}

#[test]
fn valid() {
    let sms_resource = Sms::new(get_client());
    let sms_result = sms_resource.dispatch(SmsParams{
        delay: Some("2050-12-31".to_string()),
        files: None,
        flash: None,
        foreign_id: None,
        from: None,
        label: None,
        text: "test".to_string(),
        to: "491716992343".to_string(),
        udh: None,
        ttl: None,
        performance_tracking: None,
    });
    assert!(sms_result.is_ok());
    let sms_response = sms_result.unwrap();
    let msg = sms_response.messages.first().unwrap().clone();
    let id = msg.id.unwrap();
    let ids = vec![id; 2];
    let msg_count = ids.len();
    let result = init_resource().get(StatusParams {
        msg_ids: ids.clone(),
    });
    assert!(result.is_ok());

    let statuses = result.unwrap();
    assert_eq!(msg_count, statuses.len());

    for status in statuses.into_iter() {
        assert!(ids.contains(&status.id));
        assert_eq!("TRANSMITTED", status.status.unwrap());
        assert_eq!("0000-00-00 00:00:00.000", status.status_time.unwrap());
    }

    sms_resource.delete(DeleteSmsParams{ ids }).unwrap();
}