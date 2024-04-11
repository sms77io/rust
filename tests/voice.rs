use testutil::*;
use seven_client::voice::{Voice, VoiceParams};

mod testutil;

fn init() -> Voice{
    Voice::new(get_client())
}
#[test]
fn text() {
    let params = VoiceParams {
        from: None,
        ringtime: Option::from(30),
        text: "Hello friend!".to_string(),
        to: "+491716992343".to_string(),
    };
    assert!(init().dispatch(params).is_ok());
}

#[test]
fn ssml() {
    let params = VoiceParams {
        from: None,
        ringtime: Option::from(30),
        text: "<voice name=\"de-de-female\">The total is 13.50 Euros.</voice>".to_string(),
        to: "+491716992343".to_string(),
    };
    assert!(init().dispatch(params).is_ok());
}
