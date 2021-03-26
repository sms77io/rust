use sms77_client::balance::Balance;
use testutil::*;

mod testutil;

#[test]
fn balance() {
    assert!(Balance::new(get_client()).get().is_ok());
}