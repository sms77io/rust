use seven_client::balance::Balance;
use testutil::*;

mod testutil;

#[test]
fn balance() {
    let ok = Balance::new(get_client()).get().is_ok();
    assert!(ok);
}
