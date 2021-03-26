use testutil::*;
use sms77_client::voice::{Voice, VoiceParams};

mod testutil;

fn client() -> Voice {
    Voice::new(get_client())
}

fn params() -> VoiceParams {
    VoiceParams {
        from: Option::from("Sms77.io".to_string()),
        text: "HI2U!".to_string(),
        to: "+491716992343".to_string(),
        xml: None,
    }
}

#[test]
fn text() {
    assert!(client().text(params()).is_ok());
}

#[test]
fn json() {
    assert!(client().json(params()).is_ok());
}