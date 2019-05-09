use actix_web::actix::Message;

use commons::Result;

use crate::models::domains::ImmortalUser;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub remember: bool,
    pub nickname: String,
    pub password: String,
}

impl Message for LoginRequest {
    type Result = Result<ImmortalUser>;
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub struct GetPrivileges {
    pub user_id: i32,
}

impl Message for GetPrivileges {
    type Result = Result<HashMap<String, i32>>;
}
