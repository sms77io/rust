use testutil::*;

mod testutil;

#[test]
fn instance() {
    let client = get_client();
    assert_eq!(get_api_key().to_string(), client.api_key);
    assert_eq!(SENT_WITH, client.sent_with);
}