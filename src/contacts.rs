use serde_aux::prelude::{deserialize_number_from_string};
use crate::{client::Client, OrderDirection, PagingMetadata};
use ureq::{Error, Response};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize)]
struct ContactUpsertParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<u64>,
    #[serde(flatten)]
    pub properties: ContactProperties,
}
impl ContactUpsertParams {
    fn new(contact: Contact) -> Self {
        return ContactUpsertParams{
            avatar: contact.avatar,
            groups: contact.groups,
            properties: contact.properties,
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<u64>,
    #[serde(deserialize_with = "deserialize_number_from_string", skip_serializing_if = "is_default_id")]
    pub id: u64,
    #[serde(skip_serializing)]
    pub initials: ContactInitials,
    pub properties: ContactProperties,
    #[serde(skip_serializing)]
    pub validation: ContactValidation,
}
impl Contact {
    pub fn new(properties: ContactProperties, avatar: Option<String>, groups: Option<Vec<u64>>) -> Self {
        return Contact{
            avatar,
            created: None,
            groups: groups.unwrap_or_default(),
            id: Default::default(),
            initials: ContactInitials::new(),
            properties,
            validation: ContactValidation::new(),
        }
    }
}

fn is_default_id(b: impl std::borrow::Borrow<u64>) -> bool {
    let default: u64 = Default::default();
    b.borrow() == &default
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactValidation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
impl ContactValidation {
    pub fn new() -> Self {
        return ContactValidation{
            state: None,
            timestamp: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactInitials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
}
impl ContactInitials {
    pub fn new() -> Self {
        return ContactInitials{
            color: None,
            initials: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    //#[serde(flatten)]
    //pub custom_fields: HashMap<String, Value>, // TODO: flatten?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_option_string_from_number")]
    pub home_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_option_string_from_number")]
    pub mobile_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_option_string_from_number")]
    pub postal_code: Option<String>,
}
impl ContactProperties {
    pub fn new() -> Self {
        return ContactProperties{
            address: None,
            birthday: None,
            city: None,
            //custom_fields: Default::default(),
            email: None,
            firstname: None,
            home_number: None,
            lastname: None,
            mobile_number: None,
            notes: None,
            postal_code: None,
        }
    }
}

pub fn deserialize_option_string_from_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Null,
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => Ok(Some(s)),
        StringOrNumber::Number(i) => Ok(Some(i.to_string())),
        StringOrNumber::Null => Ok(None),
    }
}

#[derive(Default, Serialize)]
pub struct ContactsListParams {
    pub group_id: Option<u64>,
    pub limit: Option<u16>,
    pub offset: Option<u64>,
    pub order_by: Option<String>,
    pub order_direction: Option<OrderDirection>,
    pub search: Option<String>,
}
impl ContactsListParams {
    pub fn new() -> Self {
        return ContactsListParams {
            group_id: None,
            limit: None,
            offset: None,
            order_by: None,
            order_direction: None,
            search: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ContactsListResponse {
    pub data: Vec<Contact>,
    #[serde(rename = "pagingMetadata")]
    pub paging_metadata: PagingMetadata,
}

pub struct Contacts {
    client: Client,
}

impl Contacts {
    pub fn new(client: Client) -> Self {
        Contacts {
            client,
        }
    }

    pub fn create(&self, contact: Contact) -> Result<Contact, Error> {
        Ok(self.client.post("contacts")
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_string(&serde_qs::to_string(&ContactUpsertParams::new(contact)).unwrap())?
            .into_json()?)
    }

    pub fn delete(&self, id: u64) -> Result<Response, Error> {
        Ok(self.client.delete(&*format!("contacts/{}", id)).call()?)
    }

    pub fn list(&self, params: ContactsListParams) -> Result<ContactsListResponse, Error> {
        Ok(self.client.get(&*format!("contacts?{}", serde_qs::to_string(&params).unwrap()))
            .call()
            .unwrap()
            .into_json()?)
    }

    pub fn one(&self, id: u64) -> Result<Contact, Error> {
        Ok(self.client.get(&*format!("contacts/{}", id))
            .call()?
            .into_json()?)
    }

    pub fn update(&self, contact: Contact) -> Result<Contact, Error> {
        Ok(self.client.patch(&*format!("contacts/{}", contact.id))
            .send_string(&*serde_qs::to_string(&ContactUpsertParams::new(contact)).unwrap())?
            .into_json()?)
    }
}