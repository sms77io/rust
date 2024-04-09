use testutil::*;
use seven_client::lookup::{Lookup, LookupParams, RcsCapability};

mod testutil;

fn client() -> Lookup {
    Lookup::new(get_client())
}

#[test]
fn cnam() {
    let res = client().cnam(LookupParams {
        number: "+491716992343".to_string()
    });
    assert!(res.is_ok());
}

#[test]
fn format() {
    let res = client().format(LookupParams {
        number: "+491716992343".to_string()
    });
    assert!(res.is_ok());

    let format = res.unwrap();
    assert!(!format.carrier.is_empty());
    assert!(!format.country_code.is_empty());
    assert!(!format.country_iso.is_empty());
    assert!(!format.country_name.is_empty());
    assert!(!format.international.is_empty());
    assert!(!format.international_formatted.is_empty());
    assert!(!format.national.is_empty());
    assert!(!format.network_type.is_empty());
    assert!(format.success);
}

#[test]
fn hlr() {
    let res = client().hlr(LookupParams {
        number: "+491716992343".to_string()
    });
    assert!(res.is_ok());
}

#[test]
fn mnp() {
    let res = client().mnp(LookupParams {
        number: "+491716992343".to_string()
    });
    assert!(res.is_ok());
}

#[test]
fn rcs() {
    let res = client().rcs(LookupParams {
        number: "+491716992343".to_string()
    });
    assert!(res.is_ok());

    let response = res.unwrap();
    assert!(!response.carrier.is_empty());
    assert!(!response.country_code.is_empty());
    assert!(!response.country_iso.is_empty());
    assert!(!response.country_name.is_empty());
    assert!(!response.network_type.is_empty());
    assert!(!response.international.is_empty());
    assert!(!response.international_formatted.is_empty());
    assert!(!response.national.is_empty());
    assert!(response.success);

    for name in response.rcs_capabilities.into_iter() {
        assert!(RcsCapability::is_valid(name.as_str()))
    }
}