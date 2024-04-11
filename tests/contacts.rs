use std::option::Option;
use std::convert::TryFrom;
use ureq::Error;
use testutil::*;
use seven_client::contacts::{Contacts, ContactsListParams, Contact, ContactInitials, ContactProperties, ContactValidation};

mod testutil;

fn init_client() -> Contacts {
    Contacts::new(get_client())
}

#[test]
fn update() {
    let original_contact_res = create_contact();
    assert!(original_contact_res.is_ok());
    let original_contact = original_contact_res.unwrap();

    let client = init_client();
    let updated_res = client.update(Contact{
        avatar: Option::from("https://avatars.githubusercontent.com/u/37155205".to_string()),
        created: original_contact.created,
        groups: original_contact.groups,
        id: original_contact.id,
        initials: original_contact.initials,
        properties: ContactProperties {
            address: Option::from("Address".to_string()),
            birthday: Option::from("01.01.1900".to_string()),
            city: Option::from("New City".to_string()),
            email: Option::from(create_mail()),
            firstname: Option::from("First Name".to_string()),
            home_number: Option::from(4943123456789),
            lastname:  Option::from("Lastname".to_string()),
            mobile_number: Option::from(491716992343),
            notes: Option::from("Zero Notes".to_string()),
            postal_code: Option::from("Postal Code".to_string()),
        },
        validation: original_contact.validation,
    });
    assert!(updated_res.is_ok());
    let updated_contact = updated_res.unwrap();

    let original_properties = original_contact.properties;
    assert_ne!(updated_contact.avatar.unwrap(), original_properties.address.clone().unwrap());
    assert_eq!(updated_contact.created.unwrap_or_default(), original_contact.created.clone().unwrap());
    assert_eq!(updated_contact.id.unwrap(), original_contact.id.unwrap());
    assert_ne!(updated_contact.properties.address.unwrap(), original_properties.address.unwrap());
    assert_ne!(updated_contact.properties.birthday.unwrap(), original_properties.birthday.unwrap());
    assert_ne!(updated_contact.properties.city.unwrap(), original_properties.city.unwrap());
    assert_ne!(updated_contact.properties.email.unwrap(), original_properties.email.unwrap());
    assert_ne!(updated_contact.properties.firstname.unwrap(), original_properties.firstname.unwrap());
    assert_ne!(updated_contact.properties.home_number.unwrap(), original_properties.home_number.unwrap());
    assert_ne!(updated_contact.properties.lastname.unwrap(), original_properties.lastname.unwrap());
    assert_ne!(updated_contact.properties.mobile_number.unwrap(), original_properties.mobile_number.unwrap());
    assert_ne!(updated_contact.properties.notes.unwrap(), original_properties.notes.unwrap());
    assert_ne!(updated_contact.properties.postal_code.unwrap(), original_properties.postal_code.unwrap());

    client.delete(updated_contact.id.unwrap()).unwrap();
}

#[test]
fn create() {
    let result = create_contact();
    assert!(result.is_ok());

    let contact = result.unwrap();

    assert_ne!(contact.created.unwrap_or_default(), "");
    assert!(contact.avatar.is_none());
    assert!(contact.id.unwrap() > 0);
    let properties = properties();
    assert_eq!(contact.properties.address.unwrap(), properties.address.unwrap());
    assert_eq!(contact.properties.birthday.unwrap(), properties.birthday.unwrap());
    assert_eq!(contact.properties.city.unwrap(), properties.city.unwrap());
    assert_eq!(contact.properties.email.unwrap(), properties.email.unwrap());
    assert_eq!(contact.properties.firstname.unwrap(), properties.firstname.unwrap());
    assert_eq!(contact.properties.home_number.unwrap(), properties.home_number.unwrap());
    assert_eq!(contact.properties.lastname.unwrap(), properties.lastname.unwrap());
    assert_eq!(contact.properties.mobile_number.unwrap(), properties.mobile_number.unwrap());
    assert_eq!(contact.properties.notes.unwrap(), properties.notes.unwrap());
    assert_eq!(contact.properties.postal_code.unwrap(), properties.postal_code.unwrap());

    init_client().delete(contact.id.unwrap()).unwrap();
}

#[test]
fn list() {
    let result = init_client().list(ContactsListParams {
        group_id: None,
        limit: None,
        offset: None,
        order_by: None,
        order_direction: None,
        search: None,
    });
    assert!(result.is_ok());
    let response = result.unwrap();

    for contact in response.data.into_iter() {
        assert_ne!(contact.created.unwrap(), "");
        assert_ne!(contact.avatar.unwrap(), "");
        assert!(contact.id.unwrap() >= u64::try_from(0).unwrap());
    }

}

fn properties() -> ContactProperties {
    return ContactProperties {
        address: Option::from("Address".to_string()),
        birthday: Option::from("1999-01-01".to_string()),
        city: Option::from("Kiel".to_string()),
        email: Option::from("a@seven.dev".to_string()),
        firstname: Option::from("Tommy".to_string()),
        home_number: Option::from(4943130149270),
        lastname: Option::from("Tester".to_string()),
        mobile_number: Option::from(491716992343),
        notes: Option::from("Some Notes...".to_string()),
        postal_code: Option::from("24103".to_string()),
    };
}

fn create_contact() -> Result<Contact, Error> {
    return init_client().create(Contact {
        avatar: None,
        created: None,
        groups: vec![],
        id: None,
        initials: ContactInitials { color: None, initials: None },
        properties: properties(),
        validation: ContactValidation { state: None, timestamp: None },
    });
}

#[test]
fn one() {
    let create_contact_result = create_contact();
    let contact = create_contact_result.unwrap();
    let result = init_client().one(contact.id.unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, contact.id);
}
