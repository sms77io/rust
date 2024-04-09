use seven_client::balance::Balance;
use testutil::*;

mod testutil;

#[test]
fn balance() {
    let result = Balance::new(get_client()).get();
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(!response.currency.is_empty())
}
