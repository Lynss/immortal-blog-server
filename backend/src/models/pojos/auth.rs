use std::collections::HashMap;

use actix_web::actix::Message;
use diesel::sql_types::{Array, Integer, Json as SqlJson, Record, SqlOrd, VarChar};
use serde_json::Value as Json;

use commons::Result;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub remember: bool,
    pub nickname: String,
    pub password: String,
}

impl Message for LoginRequest {
    type Result = Result<AuthInfo>;
}

#[derive(Deserialize, Serialize)]
pub struct Privileges {
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub privileges: Privileges,
}

#[derive(Queryable, QueryableByName,Deserialize, Serialize,Debug)]
pub struct AuthInfo {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Array<VarChar>"]
    pub roles: Vec<String>,
    #[sql_type = "Array<VarChar>"]
    pub permissions: Vec<String>,
}
