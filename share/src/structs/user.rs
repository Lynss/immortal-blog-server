use actix::Message;

use crate::{
    domains::ImmortalUser,
    schema::immortal_users,
    structs::{TableRequest, TableResponse, TimeRange, UserAndPrivilegesInfo, UserInfo},
};
use common::Result;

#[derive(Deserialize, Serialize, Default)]
pub struct UserConditions {
    pub nickname: Option<String>,
    pub created_at: Option<TimeRange>,
    pub updated_at: Option<TimeRange>,
    pub email: Option<String>,
    pub roles: Option<Vec<i32>>,
}

impl Message for UserConditions {
    type Result = Result<Vec<ImmortalUser>>;
}

#[derive(Deserialize, Serialize)]
pub struct CheckRepeatedUser(pub String);

impl Message for CheckRepeatedUser {
    type Result = Result<Option<ImmortalUser>>;
}

impl Message for TableRequest<UserConditions> {
    type Result = Result<TableResponse<UserInfo>>;
}

pub struct GetAuthorOptions;

impl Message for GetAuthorOptions {
    type Result = Result<Vec<UserAndPrivilegesInfo>>;
}

#[derive(Deserialize)]
pub struct ForbiddenUsers {
    pub ids: Vec<i32>,
}

impl Message for ForbiddenUsers {
    type Result = Result<usize>;
}

#[derive(Deserialize)]
pub struct FindUserByName {
    pub nickname: String,
}

impl Message for FindUserByName {
    type Result = Result<ImmortalUser>;
}

#[derive(Deserialize)]
pub struct ActivatedUser {
    pub id: i32,
}

pub struct FindUsers {
    pub ids: Vec<i32>,
}

impl Message for FindUsers {
    type Result = Result<Vec<ImmortalUser>>;
}

#[derive(Deserialize)]
pub struct TokenBox {
    pub token: String,
}

pub struct ActivatingUser {
    pub id: i32,
    pub roles: Vec<i32>,
}

impl Message for ActivatingUser {
    type Result = Result<usize>;
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "immortal_users"]
pub struct UserSettingsInfo {
    pub nickname: String,
    pub email: String,
    pub phone: Option<String>,
    pub sex: i32,
    pub roles: Vec<i32>,
    pub avatar: Option<String>,
}

pub struct UserSettingsUpdate(pub i32, pub UserSettingsInfo);

impl Message for UserSettingsUpdate {
    type Result = Result<usize>;
}
