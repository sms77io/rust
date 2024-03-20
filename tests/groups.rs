use testutil::*;
use seven_client::groups::{Group, Groups};

mod testutil;

fn init_client() -> Groups {
    Groups::new(get_client())
}

#[test]
fn create() {
    let name = "Peter".to_string();
    let params = Group {
        id: None,
        created: None,
        name: name.clone(),
        members_count: None
    };
    let client = init_client();
    let create_result = client.create(params);
    assert!(create_result.is_ok());

    let group = create_result.unwrap();
    assert_eq!(group.members_count.unwrap_or_default(), 0);
    assert_eq!(group.name, name);

    client.delete(group.id.unwrap_or_default(), false).unwrap();
}

#[test]
fn delete() {
    let params = Group {
        id: None,
        created: None,
        name: "Peter".to_string(),
        members_count: None
    };
    let client = init_client();
    let create_result = client.create(params);
    let group = create_result.unwrap();

    let delete_result = client.delete(group.id.unwrap_or_default(), false);
    assert!(delete_result.is_ok());

    let response = delete_result.unwrap();
    assert_eq!(response.success, true);
}

#[test]
fn get() {
    let name = "Peter".to_string();
    let params = Group {
        id: None,
        created: None,
        name: name.clone(),
        members_count: None
    };
    let client = init_client();
    let create_result = client.create(params);
    let group = create_result.unwrap();

    let result = client.get(group.id.unwrap_or_default());
    let response = result.unwrap();

    assert_eq!(response.name, name.clone());
    assert_eq!(response.members_count.unwrap(), 0);
    assert_ne!(response.created.unwrap(), "");
    assert!(response.id.unwrap() > 0);
}

#[test]
fn all() {
    let client = init_client();

    client.create(Group {
        id: None,
        created: None,
        name: "Peter".to_string(),
        members_count: None,
    }).unwrap();

    let result = client.all();
    let response = result.unwrap();
    assert_eq!(response.paging_metadata.offset, 0);

    for group in response.data.into_iter() {
        assert_ne!(group.name, "");
        assert_ne!(group.created.unwrap(), "");
        assert!(group.id.unwrap() > 0);
    }
}

#[test]
fn update() {
    let client = init_client();
    let mut old_group = client.create(Group {
        id: None,
        created: None,
        name: "Peter".to_string(),
        members_count: None,
    }).unwrap();
    let old_name = old_group.name;
    old_group.name = "Tommy".to_string();
    let new_group = client.update(old_group).unwrap();

    assert_ne!(new_group.name, old_name);
}
