use testutil::{get_client};
use seven_client::subaccounts::{AutoChargeParams, CreateParams, DeleteParams, Subaccounts, TransferCreditsParams};

mod testutil;

fn init_client() -> Subaccounts {
    Subaccounts::new(get_client())
}

#[test]
fn read() {
    let res = init_client().read();

    assert!(res.is_ok());
}

#[test]
fn create() {
    let params = CreateParams {
        email: "a@b.de".to_string(),
        name: "Ay Oy".to_string(),
    };
    let res = init_client().create(params);

    assert!(res.is_ok());
}

#[test]
fn delete() {
    let params = DeleteParams {
        id: 965887,
    };
    let res = init_client().delete(params);

    assert!(res.is_ok());
}

#[test]
fn transfer_credits() {
    let params = TransferCreditsParams {
        amount: 0.1,
        id: 941955,
    };
    let res = init_client().transfer_credits(params);

    assert!(res.is_ok());
}


#[test]
fn auto_charge() {
    let params = AutoChargeParams {
        amount: 0.2,
        id: 941955,
        threshold: 1.0,
    };
    let res = init_client().auto_charge(params);

    assert!(res.is_ok());
}

