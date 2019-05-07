use actix_web::{actix::Message, Result};
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct ImmortalUser {
    pub id: i32,
    pub nickname: String,
    pub password: String,
    pub role: Vec<String>,
    pub email: String,
    pub phone: Option<String>,
    pub sex: i32,
    pub created_at: NaiveDateTime,
    pub avatar: String,
}

pub struct GetUser {
    pub phone: Option<String>,
}

impl Message for GetUser {
    type Result = Result<Vec<ImmortalUser>>;
}
