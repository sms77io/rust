use crate::{client::Client, OrderDirection, PagingMetadata};
use ureq::{Error, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Contact {
    pub avatar: Option<String>,
    pub created: Option<String>,
    pub groups: Vec<u64>,
    pub id: Option<u64>,
    pub initials: ContactInitials,
    pub properties: ContactProperties,
    pub validation: ContactValidation,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactValidation {
    pub state: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactInitials {
    pub color: Option<String>,
    pub initials: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactProperties {
    pub address: Option<String>,
    pub birthday: Option<String>,
    pub city: Option<String>,
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub home_number: Option<u64>,
    pub lastname: Option<String>,
    pub mobile_number: Option<u64>,
    pub notes: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Default)]
pub struct ContactsListParams {
    pub group_id: Option<u64>,
    pub limit: Option<u16>,
    pub offset: Option<u64>,
    pub order_by: Option<String>,
    pub order_direction: Option<OrderDirection>,
    pub search: Option<String>,
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
        let res = self.client.post("contacts")
            .send_form(&[
                ("address", &*contact.properties.address.unwrap_or_default()),
                ("avatar", &*contact.avatar.unwrap_or_default()),
                ("birthday", &*contact.properties.birthday.unwrap_or_default()),
                ("city", &*contact.properties.city.unwrap_or_default()),
                ("email", &*contact.properties.email.unwrap_or_default()),
                ("firstname", &*contact.properties.firstname.unwrap_or_default()),
                //("groups[]", &*contact.groups.map(String::as_str).join(",")),
                //("groups[]", &*contact.groups.iter().map(String::as_str).collect()),
                ("groups[]", &*contact.groups.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(&",".to_string())),
                ("home_number", &*contact.properties.home_number.unwrap_or_default().to_string()),
                ("lastname", &*contact.properties.lastname.unwrap_or_default()),
                ("mobile_number", &*contact.properties.mobile_number.unwrap_or_default().to_string()),
                ("notes", &*contact.properties.notes.unwrap_or_default()),
                ("postal_code", &*contact.properties.postal_code.unwrap_or_default()),
            ])?
            .into_json::<Contact>()?;
        Ok(res)
    }

    pub fn delete(&self, id: u64) -> Result<Response, Error> {
        let endpoint = format!("contacts/{id}");
        let res = self.client.delete(&*endpoint)
            .call()?;
        Ok(res)
    }

    pub fn list(&self, params: ContactsListParams) -> Result<ContactsListResponse, Error> {
        let res = self.client.get( "contacts")
            .query("group_id", &*params.group_id.unwrap_or_default().to_string())
            .query("limit", &*params.limit.unwrap_or_default().to_string())
            .query("offset", &*params.offset.unwrap_or_default().to_string())
            .query("order_by", &*params.order_by.unwrap_or_default())
            .query("order_direction", &*params.order_direction.unwrap_or_default().as_str())
            .query("search", &*params.search.unwrap_or_default())
            .call()
            .unwrap()
            .into_json::<ContactsListResponse>()?;
        Ok(res)
    }

    pub fn one(&self, id: u64) -> Result<Contact, Error> {
        let endpoint = format!("contacts/{id}");
        let res = self.client.get(&*endpoint)
            .call()?
            .into_json::<Contact>()?;
        Ok(res)
    }

    pub fn update(&self, contact: Contact) -> Result<Contact, Error> {
        let id = contact.id.unwrap();
        let res = self.client.patch(&*format!("contacts/{id}"))
            .send_form(&[
                ("address", &*contact.properties.address.unwrap_or_default()),
                ("avatar", &*contact.avatar.unwrap_or_default()),
                ("birthday", &*contact.properties.birthday.unwrap_or_default()),
                ("city", &*contact.properties.city.unwrap_or_default()),
                ("email", &*contact.properties.email.unwrap_or_default()),
                ("firstname", &*contact.properties.firstname.unwrap_or_default()),
                //("groups[]", &*contact.groups.map(String::as_str).join(",")),
                //("groups[]", &*contact.groups.iter().map(String::as_str).collect()),
                ("groups[]", &*contact.groups.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(&",".to_string())),
                ("home_number", &*contact.properties.home_number.unwrap_or_default().to_string()),
                ("lastname", &*contact.properties.lastname.unwrap_or_default()),
                ("mobile_number", &*contact.properties.mobile_number.unwrap_or_default().to_string()),
                ("notes", &*contact.properties.notes.unwrap_or_default()),
                ("postal_code", &*contact.properties.postal_code.unwrap_or_default()),
            ])?
            .into_json::<Contact>()?;
        Ok(res) // TODO: fails because returned "id" is string instead of int
    }
}