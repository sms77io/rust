use ureq::{Error};
use crate::client::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DeleteGroupResponse {
    pub success: bool,
}

#[derive(Deserialize)]
pub struct PagingMetadata {
    pub count: u8,
    pub has_more: bool,
    pub offset: u8,
    pub total: u8,
}

#[derive(Deserialize)]
pub struct GroupsResponse {
    #[serde(rename(deserialize = "pagingMetadata"))]
    pub paging_metadata: PagingMetadata,
    pub data: Vec<Group>,
}

#[derive(Deserialize, Serialize)]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_count: Option<u64>,
    pub name: String,
}

pub struct Groups {
    client: Client,
}

impl Groups {
    pub fn new(client: Client) -> Self {
        Groups {
            client,
        }
    }

    pub fn all(&self) -> Result<GroupsResponse, Error> {
        let res = self.client.request("GET", "groups")
            .call()?
            .into_json::<GroupsResponse>()?;
        Ok(res)
    }

    pub fn get(&self, id: u64) -> Result<Group, Error> {
        let endpoint = format!("groups/{id}");
        let res = self.client.request("GET", &endpoint)
            .call()?
            .into_json::<Group>()?;
        Ok(res)
    }

    pub fn create(&self, group: Group) -> Result<Group, Error> {
        let res = self.client.request("POST", "groups")
            .send_form(&[
                ("name", &*group.name),
            ])?
            .into_json::<Group>()?;
        Ok(res)
    }

    pub fn update(&self, group: Group) -> Result<Group, Error> {
        let id = group.id.unwrap();
        let endpoint = format!("groups/{id}");
        let res = self.client.request("PATCH", &endpoint)
            .send_form(&[
                ("name", &*group.name),
            ])?
            .into_json::<Group>()?;
        Ok(res)
    }

    pub fn delete(&self, id: u64, delete_contacts: bool) -> Result<DeleteGroupResponse, Error> {
        let endpoint = format!("groups/{id}");
        let res = self.client.request("DELETE", &*endpoint)
            .send_form(&[
                ("delete_contacts", if delete_contacts {"1"} else {"0"}),
            ])?
            .into_json::<DeleteGroupResponse>()?;
        Ok(res)
    }
}
