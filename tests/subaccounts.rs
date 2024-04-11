use testutil::{get_client};
use seven_client::subaccounts::{AutoChargeParams, CreateParams, DeleteParams, Subaccounts, TransferCreditsParams};
use crate::testutil::{create_mail};

mod testutil;

fn init_client() -> Subaccounts {
    Subaccounts::new(get_client())
}

#[test]
fn read() {
    let res = init_client().list();

    assert!(res.is_ok());
}

#[test]
fn create() {
    let email = create_mail();
    let name = "Ay Oy".to_string();

    let client = init_client();
    let res = client.create(CreateParams {
        email: email.clone(),
        name: name.clone(),
    });

    assert!(res.is_ok());

    let response = res.unwrap();

    let  subaccount_opt = response.subaccount;
    assert!(subaccount_opt.is_some());
    let subaccount = subaccount_opt.unwrap();

    assert_eq!(name, subaccount.contact.name);
    assert_eq!(email, subaccount.contact.email);

    client.delete(DeleteParams{ id: subaccount.id }).unwrap();
}

#[test]
fn delete() {
    let client = init_client();
    let res = client.create(CreateParams {
        email: create_mail(),
        name: "Tim Test".to_string(),
    });
    assert!(res.is_ok());

    let subaccount_opt = res.unwrap().subaccount;
    assert!(subaccount_opt.is_some());

    let result = client.delete(DeleteParams {
        id: subaccount_opt.unwrap().id,
    });
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.success);
    assert!(response.error.is_none());
}

#[test]
fn get() {
    let email = create_mail();
    let name = "Ay Oy".to_string();
    let client = init_client();
    let create_result = client.create(CreateParams {
        email: email.clone(),
        name: name.clone(),
    });
    assert!(create_result.is_ok());

    let subaccount_opt = create_result.unwrap().subaccount;
    assert!(subaccount_opt.is_some());

    let expected_id = subaccount_opt.unwrap().id;
    let actual_result = client.get(expected_id);
    assert!(actual_result.is_ok());
    let actual = actual_result.unwrap();

    assert_eq!(expected_id, actual.id);
    assert_eq!(email, actual.contact.email);
    assert_eq!(name, actual.contact.name);

    client.delete(DeleteParams{ id: expected_id }).unwrap();
}

#[test]
fn transfer_credits() {
    let client = init_client();
    let create_result = client.create(CreateParams {
        email: create_mail(),
        name: "Ay Oy".to_string(),
    });
    assert!(create_result.is_ok());

    let subaccount_opt = create_result.unwrap().subaccount;
    assert!(subaccount_opt.is_some());

    let id = subaccount_opt.unwrap().id;
    let result = client.transfer_credits(TransferCreditsParams {
        amount: 0.1,
        id,
    });
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.success);
    assert!(response.error.is_none());

    client.delete(DeleteParams{id}).unwrap();
}


#[test]
fn auto_charge() {
    let client = init_client();
    let create_result = client.create(CreateParams {
        email: create_mail(),
        name: "Ay Oy".to_string(),
    });
    assert!(create_result.is_ok());

    let subaccount_opt = create_result.unwrap().subaccount;
    assert!(subaccount_opt.is_some());

    let id = subaccount_opt.unwrap().id;
    let result = client.auto_charge(AutoChargeParams {
        amount: 0.2,
        id,
        threshold: 1.0,
    });

    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.success);
    assert!(response.error.is_none());

    client.delete(DeleteParams{id}).unwrap();
}

