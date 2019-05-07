use actix_web::actix::Message;

use commons::Result;

use crate::models::domains::ImmortalUser;

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
