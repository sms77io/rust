use std::{env};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use regex::Regex;
use testutil::*;
use seven_client::sms::{DeleteSmsParams, Sms, SmsFile, SmsParams};

mod testutil;

fn init_client() -> Sms {
    Sms::new(get_client())
}

#[test]
fn simple() {
    let delay = "2040-12-30".to_string();
    let from = "seven.io".to_string();
    let label = "Label".to_string();
    let text = "Hi there!".to_string();
    let to = "491716992343".to_string();
    let params = SmsParams {
        delay: Some(delay),
        files: None,
        flash: Some(true),
        foreign_id: Some("Foreign ID".to_string()),
        from: Some(from.clone()),
        label: Some(label.clone()),
        performance_tracking: Some(true),
        text: text.clone(),
        to: to.clone(),
        ttl: None,
        udh: None,
    };
    let client = init_client();

    let result = client.dispatch(params);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.success, "100");
    assert_eq!(response.messages.len(), 1);

    let msg = response.messages.first().unwrap();
    let id = msg.clone().id.unwrap();

    assert!(msg.error.is_none());
    assert!(msg.error_text.is_none());
    assert_eq!(msg.encoding, "gsm");
    assert!(!msg.is_binary);
    assert_eq!(msg.label, Some(label));
    assert_eq!(msg.parts, 1);
    assert_eq!(msg.recipient, to);
    assert_eq!(msg.sender, from);
    assert!(msg.success);
    assert_eq!(msg.text, text);
    assert!(msg.udh.is_none());

    client.delete(DeleteSmsParams{ids: vec![id]}).unwrap();
}

#[test]
fn files() {
    let filename: &'static str = "dummy.pdf";
    let text = format!("Hi, our TOS changed, please see [[{}]].", filename);
    let mut buf = vec![];
    File::open(
        PathBuf::from(env!("CARGO_MANIFEST_DIR").to_string()).join("tests").join(filename)
    ).unwrap().read_to_end(&mut buf).unwrap();
    let files = vec![
        SmsFile::new(SmsFile{
            contents: String::from_utf8_lossy(&buf).to_string(),
            name: filename.to_string(),
            password: Some("".to_string()),
            validity: Some(30),
        })
    ];
    let file_count = files.len();
    let client = init_client();
    let params = SmsParams {
        delay: Some("2040-12-30".to_string()),
        files: Some(files),
        flash: None,
        foreign_id: None,
        from: None,
        label: None,
        performance_tracking: None,
        text: text.clone(),
        to: "491716992343".to_string().clone(),
        ttl: None,
        udh: None,
    };
    let result = client.dispatch(params);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.messages.len(), 1);

    let msg = response.messages.first().unwrap();
    assert_ne!(msg.text, text);

    assert_eq!(
        file_count,
        Regex::new(r"[[(.+)]]").unwrap().captures(&*text).unwrap().len()
    );

    assert_eq!(
        file_count,
        Regex::new(r"https://svn.me/[A-Za-z0-9]+").unwrap().captures(&*msg.text).unwrap().len()
    );

    client.delete(DeleteSmsParams{ids: vec![msg.clone().id.unwrap()]}).unwrap();
}
