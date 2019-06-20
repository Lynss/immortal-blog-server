use actix::Message;

use crate::{
    domains::Category,
    schema::categories,
    structs::{TableRequest, TableResponse, TimeRange},
};
use common::Result;

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoryConditions {
    pub name: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: Option<TimeRange>,
    pub updated_at: Option<TimeRange>,
}

impl Default for CategoryConditions {
    fn default() -> Self {
        CategoryConditions {
            name: None,
            created_by: None,
            updated_by: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl Message for TableRequest<CategoryConditions> {
    type Result = Result<TableResponse<Category>>;
}

#[derive(Deserialize)]
pub struct CategoryCreateInfo {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "categories"]
pub struct CategoryCreate {
    pub name: String,
    pub description: Option<String>,
    pub created_by: String,
    pub updated_by: String,
}

impl CategoryCreate {
    pub fn new(info: CategoryCreateInfo, user: String) -> Self {
        CategoryCreate {
            created_by: user.clone(),
            updated_by: user,
            name: info.name,
            description: info.description,
        }
    }
}

impl Message for CategoryCreate {
    type Result = Result<()>;
}

#[derive(Deserialize)]
pub struct CategoryDelete {
    pub ids: Vec<i32>,
}

impl Message for CategoryDelete {
    type Result = Result<usize>;
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "categories"]
pub struct CategoryUpdateInfo {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub struct CategoryUpdate(pub i32, pub CategoryUpdateInfo);

impl Message for CategoryUpdate {
    type Result = Result<()>;
}
