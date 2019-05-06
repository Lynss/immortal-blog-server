use actix_web::{actix::Message, Result};
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct ImmortalUser {
    id: i32,
    nickname: String,
    password: String,
    role: Vec<String>,
    email: String,
    phone: Option<String>,
    sex: i32,
    created_at: NaiveDateTime,
    avatar: String,
}

pub struct GetUser {
    pub phone: Option<String>,
}

impl Message for GetUser {
    type Result = Result<Vec<ImmortalUser>>;
}
