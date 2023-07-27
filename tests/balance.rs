use seven_client::balance::Balance;
use testutil::*;

mod testutil;

#[test]
fn balance() {
    let res = Balance::new(get_client()).get();
    assert!(res.is_ok());
}

#[test]
fn json() {
    let res = Balance::new(get_client()).json();
    assert!(res.is_ok());
}
