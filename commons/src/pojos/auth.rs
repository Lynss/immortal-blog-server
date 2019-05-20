use actix_web::actix::Message;
use diesel::sql_types::{Array, Integer, Record, VarChar, Timestamp};
use chrono::NaiveDateTime;

use crate::{Result, schema::immortal_users};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub remember: bool,
    pub nickname: String,
    pub password: String,
}

impl Message for LoginRequest {
    type Result = Result<AuthInfo>;
}

#[derive(Serialize)]
pub struct UserInfo {
    pub nickname: String,
    pub email: String,
    pub phone: String,
    pub avatar: String,
    pub sex: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct Privileges {
    pub roles: Vec<String>,
    pub permissions: HashMap<String, i32>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_info: UserInfo,
    pub privileges: Privileges,
}

#[derive(Queryable, QueryableByName, Deserialize, Serialize, Debug)]
pub struct AuthInfo {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "VarChar"]
    pub nickname: String,
    #[sql_type = "VarChar"]
    pub email: String,
    #[sql_type = "VarChar"]
    pub phone: String,
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
