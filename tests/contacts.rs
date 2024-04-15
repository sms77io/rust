use testutil::*;
use seven_client::contacts::{Contacts, ContactsListParams, Contact, ContactProperties};

mod testutil;

fn init() -> Contacts {
    Contacts::new(get_client())
}

#[test]
fn update() {
    let original_contact = init().create(Contact::new(ContactProperties::new(), None, None)).unwrap();
    let id_original = original_contact.id;
    let created_original = &original_contact.created;
    let original_properties = original_contact.properties;

    let address = &Some("Address".to_string());
    let birthday = &Some("01.01.1900".to_string());
    let city = &Some("New City".to_string());
    let email = &Some(create_mail());
    let firstname = &Some("First Name".to_string());
    let home_number = Some("4943123456789".to_string());
    let lastname = &Some("Lastname".to_string());
    let mobile_number = Some("491716992343".to_string());
    let notes = &Some("Zero Notes".to_string());
    let postal_code = &Some("Postal Code".to_string());
    let avatar = &Some("https://avatars.githubusercontent.com/u/37155205".to_string());
    let updated_contact = init().update(Contact{
        avatar: avatar.clone(),
        created: created_original.clone(),
        groups: original_contact.groups,
        id: id_original,
        initials: original_contact.initials,
        properties: ContactProperties {
            address: address.clone(),
            birthday: birthday.clone(),
            city: city.clone(),
            //custom_fields: Default::default(),
            email: email.clone(),
            firstname: firstname.clone(),
            home_number: home_number.clone(),
            lastname: lastname.clone(),
            mobile_number: mobile_number.clone(),
            notes: notes.clone(),
            postal_code: postal_code.clone(),
        },
        validation: original_contact.validation,
    }).unwrap();

    assert_ne!(*avatar, original_contact.avatar);
    assert_eq!(updated_contact.created, created_original.clone());
    assert_eq!(updated_contact.id, id_original);
    assert_ne!(*address, original_properties.address);
    assert_ne!(*birthday, original_properties.birthday);
    assert_ne!(*city, original_properties.city);
    assert_ne!(*email, original_properties.email);
    assert_ne!(*firstname, original_properties.firstname);
    assert_ne!(home_number, original_properties.home_number);
    assert_ne!(*lastname, original_properties.lastname);
    assert_ne!(mobile_number, original_properties.mobile_number);
    assert_ne!(*notes, original_properties.notes);
    assert_ne!(*postal_code, original_properties.postal_code);

    init().delete(updated_contact.id).unwrap();
}

#[test]
fn create() {
    let address = &Some("Address".to_string());
    let birthday = &Some("1999-01-01".to_string());
    let city = &Some("Kiel".to_string());
    //let custom_fields: HashMap<String, Value> = Default::default();
    let email = &Some("a@seven.dev".to_string());
    let firstname = &Some("Tommy".to_string());
    let home_number = &Some("4943130149270".to_string());
    let lastname = &Some("Tester".to_string());
    let mobile_number = &Some("491716992343".to_string());
    let notes = &Some("Some Notes...".to_string());
    let postal_code = &Some("24103".to_string());

    let contact = init().create(Contact::new(ContactProperties {
        address: address.clone(),
        birthday: birthday.clone(),
        city: city.clone(),
        // custom_fields: custom_fields.clone(),
        email: email.clone(),
        firstname: firstname.clone(),
        home_number: home_number.clone(),
        lastname: lastname.clone(),
        mobile_number: mobile_number.clone(),
        notes: notes.clone(),
        postal_code: postal_code.clone(),
    }, None, None)).unwrap();

    assert!(contact.avatar.is_none());
    assert!(contact.created.is_some());
    assert!(contact.id > 0);

    println!("{:?}", contact);

    assert_eq!(contact.properties.address, *address);
    assert_eq!(contact.properties.birthday, *birthday);
    assert_eq!(contact.properties.city, *city);
    //assert_eq!(contact.properties.custom_fields, custom_fields.clone());
    assert_eq!(contact.properties.email, *email);
    assert_eq!(contact.properties.firstname, *firstname);
    assert_eq!(contact.properties.home_number, *home_number);
    assert_eq!(contact.properties.lastname, *lastname);
    assert_eq!(contact.properties.mobile_number, *mobile_number);
    assert_eq!(contact.properties.notes, *notes);
    assert_eq!(contact.properties.postal_code, *postal_code);

    init().delete(contact.id).unwrap();
}

#[test]
fn list() {
    let contact = init().create(Contact::new(ContactProperties::new(), None, None)).unwrap();
    let result = init().list(ContactsListParams::new());

    for contact in result.unwrap().data.into_iter() {
        assert!(contact.created.is_some());
        assert_ne!(contact.id, 0);
    }

    init().delete(contact.id).unwrap();
}

#[test]
fn one() {
    let id = init().create(Contact::new(ContactProperties::new(), None, None)).unwrap().id;
    let response = init().one(id).unwrap();
    assert_eq!(response.id, id);

    init().delete(id).unwrap();
}