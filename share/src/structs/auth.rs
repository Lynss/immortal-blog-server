use actix::Message;
use chrono::NaiveDateTime;
use diesel::sql_types::{Array, Bool, Integer, Nullable, Record, Timestamp, VarChar};

use crate::{
    domains::{ImmortalUser, Role},
    schema::immortal_users,
};
use common::{utils, Result};
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

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct UserInfo {
    pub id: i32,
    pub nickname: String,
    pub roles: Vec<i32>,
    pub email: String,
    pub phone: Option<String>,
    pub avatar: String,
    pub activated: bool,
    pub sex: i32,
    #[serde(with = "utils::date_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "utils::date_format")]
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
            roles,
            activated,
        }: ImmortalUser,
    ) -> Self {
        UserInfo {
            nickname,
            roles,
            activated,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Privileges {
    pub roles: Vec<String>,
    pub permissions: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize, Debug)]
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
            role_ids,
            activated,
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
            activated,
            roles: role_ids,
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

#[derive(Queryable, QueryableByName, Deserialize, Serialize, Debug, Clone)]
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
    #[sql_type = "Array<Integer>"]
    pub role_ids: Vec<i32>,
    #[sql_type = "Bool"]
    pub activated: bool,
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

pub struct GetRoleOptions;

impl Message for GetRoleOptions {
    type Result = Result<Vec<Role>>;
}
