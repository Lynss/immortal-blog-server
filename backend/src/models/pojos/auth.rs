use crate::models::domains::ImmortalUser;
use commons::Result;

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
