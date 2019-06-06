use actix::Message;
use chrono::NaiveDateTime;
use diesel::sql_types::{Array, Integer, Nullable, Record, Timestamp, VarChar};

use crate::{domains::ImmortalUser, schema::immortal_users};
use common::Result;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub remember: bool,
    pub nickname: String,
    pub password: String,
}

impl Message for LoginRequest {
    type Result = Result<UserId>;
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub nickname: String,
    pub email: String,
    pub phone: Option<String>,
    pub avatar: String,
    pub sex: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ImmortalUser> for UserInfo {
    fn from(
        ImmortalUser {
            nickname,
            id,
            email,
            phone,
            avatar,
            sex,
            created_at,
            updated_at,
            password: _,
            roles: _,
        }: ImmortalUser,
    ) -> Self {
        UserInfo {
            nickname,
            id,
            email,
            phone,
            avatar,
            sex,
            created_at,
            updated_at,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Privileges {
    pub roles: Vec<String>,
    pub permissions: HashMap<String, i32>,
}

pub struct UserAndPrivilegesInfo(pub UserInfo, pub Privileges);

impl From<AuthInfo> for UserAndPrivilegesInfo {
    fn from(
        AuthInfo {
            id,
            email,
            nickname,
            phone,
            avatar,
            created_at,
            updated_at,
            sex,
            roles,
            permissions,
        }: AuthInfo,
    ) -> Self {
        //transform vec to map structure
        let permissions = HashMap::from_iter(permissions);
        //get privileges of current user
        let privileges = Privileges { roles, permissions };
        let user_info = UserInfo {
            id,
            email,
            nickname,
            phone,
            avatar,
            created_at,
            updated_at,
            sex,
        };
        UserAndPrivilegesInfo(user_info, privileges)
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_info: UserInfo,
    pub privileges: Privileges,
}

#[derive(Deserialize, Serialize)]
pub struct UserId {
    pub id: i32,
}

impl Message for UserId {
    type Result = Result<UserAndPrivilegesInfo>;
}

#[derive(Queryable, QueryableByName, Deserialize, Serialize, Debug)]
pub struct AuthInfo {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "VarChar"]
    pub nickname: String,
    #[sql_type = "VarChar"]
    pub email: String,
    #[sql_type = "Nullable<VarChar>"]
    pub phone: Option<String>,
    #[sql_type = "VarChar"]
    pub avatar: String,
    #[sql_type = "Integer"]
    pub sex: i32,
    #[sql_type = "Timestamp"]
    pub created_at: NaiveDateTime,
    #[sql_type = "Timestamp"]
    pub updated_at: NaiveDateTime,
    #[sql_type = "Array<VarChar>"]
    pub roles: Vec<String>,
    #[sql_type = "Array<Record<(VarChar,Integer)>>"]
    pub permissions: Vec<(String, i32)>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "immortal_users"]
pub struct RegisterRequest {
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub sex: i32,
}

impl Message for RegisterRequest {
    type Result = Result<()>;
}
