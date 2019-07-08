use crate::schema::roles;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Identifiable)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
