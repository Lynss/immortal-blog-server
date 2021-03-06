use chrono::NaiveDateTime;

use crate::schema::immortal_users;

#[derive(Queryable, Serialize, Identifiable)]
pub struct ImmortalUser {
    pub id: i32,
    pub nickname: String,
    pub password: String,
    pub roles: Vec<i32>,
    pub email: String,
    pub phone: Option<String>,
    pub sex: i32,
    pub activated: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub avatar: String,
}
