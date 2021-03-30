use testutil::*;
use sms77_client::contacts::{Contacts, ContactsReadParams, ContactDeleteParams, ContactEditParams};

mod testutil;

fn init_client() -> Contacts {
    Contacts::new(get_client())
}

#[test]
fn read() {
    assert!(init_client().read(ContactsReadParams { id: None }).is_ok());
}

#[test]
fn read_json() {
    assert!(init_client().read_json(ContactsReadParams { id: None }).is_ok());
}

#[test]
fn create() {
    assert!(init_client().create().is_ok());
}

#[test]
fn create_json() {
    assert!(init_client().create_json().is_ok());
}

#[test]
fn edit() {
    assert!(init_client().edit(ContactEditParams {
        empfaenger: None,
        id: 3172517,
        nick: None,
    }).is_ok());
}

#[test]
fn edit_json() {
    assert!(init_client().edit_json(ContactEditParams {
        empfaenger: None,
        id: 3172517,
        nick: None,
    }).is_ok());
}

#[test]
fn delete() {
    assert!(init_client().delete(ContactDeleteParams {
        id: 4848431,
    }).is_ok());
}