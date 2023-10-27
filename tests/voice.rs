use testutil::*;
use seven_client::voice::{Voice, VoiceParams};

mod testutil;

fn client() -> Voice {
    Voice::new(get_client())
}

fn params() -> VoiceParams {
    VoiceParams {
        debug: None,
        from: Option::from("seven.io".to_string()),
        ringtime: Option::from(30),
        text: "HI2U!".to_string(),
        to: "+49179876543210".to_string(),
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
