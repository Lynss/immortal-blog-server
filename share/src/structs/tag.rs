use actix::Message;

use crate::{
    domains::Tag,
    schema::tags,
    structs::{TableRequest, TableResponse, TimeRange},
};
use common::Result;

#[derive(Deserialize, Serialize, Debug)]
pub struct TagConditions {
    pub name: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: Option<TimeRange>,
    pub updated_at: Option<TimeRange>,
}

impl Default for TagConditions {
    fn default() -> Self {
        TagConditions {
            name: None,
            created_by: None,
            updated_by: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl Message for TableRequest<TagConditions> {
    type Result = Result<TableResponse<Tag>>;
}

#[derive(Deserialize)]
pub struct TagCreateInfo {
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "tags"]
pub struct TagCreate {
    pub name: String,
    pub color: String,
    pub created_by: String,
    pub updated_by: String,
}

impl TagCreate {
    pub fn new(info: TagCreateInfo, user: String) -> Self {
        TagCreate {
            created_by: user.clone(),
            updated_by: user,
            name: info.name,
            color: info.color,
        }
    }
}

impl Message for TagCreate {
    type Result = Result<()>;
}

#[derive(Deserialize)]
pub struct TagDelete {
    pub ids: Vec<i32>,
}

impl Message for TagDelete {
    type Result = Result<usize>;
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "tags"]
pub struct TagUpdateInfo {
    pub name: Option<String>,
    pub color: Option<String>,
}

pub struct TagUpdate(pub i32, pub TagUpdateInfo);

impl Message for TagUpdate {
    type Result = Result<()>;
}
