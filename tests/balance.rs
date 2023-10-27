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
    let result = Balance::new(get_client()).json();
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.currency.is_empty(), false)
}
