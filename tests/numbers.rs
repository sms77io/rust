use testutil::*;
use seven_client::numbers::{AvailableNumbersParams, DeleteNumberParams, DeleteNumberResponse, Numbers, OrderNumberParams, OrderNumberResponse, PhoneNumberOffer, UpdateNumberParams};
use seven_client::numbers::PaymentInterval::Monthly;
use std::{thread, time};
use ureq::Error;

mod testutil;

fn init_client() -> Numbers {
    Numbers::new(get_client())
}

#[test]
fn active() {
    let client = init_client();
    let active_numbers = client.active().unwrap();

    for phone in active_numbers.active_numbers.into_iter() {
        assert_ne!(phone.country, "");
        assert_ne!(phone.created, "");
        assert_ne!(phone.number, "");
        assert_ne!(phone.friendly_name, "");
        assert_ne!(phone.billing.payment_interval, "");
        assert!(phone.billing.fees.basic_charge >= 0.0);
        assert!(phone.billing.fees.setup >= 0.0);
        assert!(phone.billing.fees.sms_mo >= 0.0);
        assert!(phone.billing.fees.voice_mo >= 0.0);
    }
}

#[test]
fn available() {
    let client = init_client();
    let result = client.available(AvailableNumbersParams {
        country: None,
        features_a2p_sms: None,
        features_sms: None,
        features_voice: None,
    });
    let response = result.unwrap();

    for group in response.available_numbers.into_iter() {
        assert_ne!(group.country, "");
        assert_ne!(group.number, "");
        assert_ne!(group.number_parsed, "");
        assert!(group.fees.sms_mo >= 0.0);
        assert!(group.fees.voice_mo >= 0.0);
        assert!(group.fees.monthly.setup >= 0.0);
        assert!(group.fees.monthly.basic_charge >= 0.0);
        assert!(group.fees.annually.setup >= 0.0);
        assert!(group.fees.annually.basic_charge >= 0.0);
    }
}

#[test]
fn order() {
    let client = init_client();
    let binding = client.available(AvailableNumbersParams {
        country: None,
        features_a2p_sms: None,
        features_sms: None,
        features_voice: None,
    }).unwrap();
    let offer = binding.available_numbers.first().unwrap();
    let number = offer.number.clone();
    let result = client.order(OrderNumberParams {
        number,
        payment_interval: Option::from(Monthly),
    });
    let response = result.unwrap();
    assert!(response.success);
    assert_eq!(response.error, None);

    clean_up(offer).unwrap();
}

#[test]
fn delete() {
    let client = init_client();
    let binding = client.available(AvailableNumbersParams {
        country: None,
        features_a2p_sms: None,
        features_sms: None,
        features_voice: None,
    }).unwrap();
    let offer = binding.available_numbers.first().unwrap();
    client.order(OrderNumberParams {
        number: offer.number.clone(),
        payment_interval: Option::from(Monthly),
    }).unwrap();

    thread::sleep(time::Duration::from_secs(1));

    let delete_res = clean_up(offer);
    assert!(delete_res.unwrap().success);
}

#[test]
fn get() {
    let client = init_client();
    let binding = client.available(AvailableNumbersParams {
        country: None,
        features_a2p_sms: None,
        features_sms: None,
        features_voice: None,
    }).unwrap();
    let offer = binding.available_numbers.first().unwrap();
    client.order(OrderNumberParams {
        number: offer.number.clone(),
        payment_interval: Option::from(Monthly),
    }).unwrap();

    let result = client.get(offer.number.clone());
    assert!(result.is_ok());
    let phone = result.unwrap();
    assert_eq!(phone.number, offer.number.clone());

    clean_up(offer).unwrap();
}

#[test]
fn update() {
    let client = init_client();
    let binding = client.available(AvailableNumbersParams {
        country: None,
        features_a2p_sms: None,
        features_sms: None,
        features_voice: None,
    }).unwrap();
    let offer = binding.available_numbers.first().unwrap();
    client.order(OrderNumberParams {
        number: offer.number.clone(),
        payment_interval: Option::from(Monthly),
    }).unwrap();

    let params = UpdateNumberParams {
        email_forward: None,
        friendly_name: Option::from("Friendly Name".to_string()),
        number: offer.number.clone(),
        sms_forward: None,
    };
    let result = client.update(params).unwrap();

    assert_eq!(result.friendly_name, params.friendly_name.unwrap());

    clean_up(offer).unwrap();
}

fn clean_up(offer: &PhoneNumberOffer) -> Result<DeleteNumberResponse, Error> {
    thread::sleep(time::Duration::from_secs(1));

    return init_client().delete(DeleteNumberParams {
        delete_immediately: Option::from(true),
        number: offer.number.clone(),
    });
}