use chrono::NaiveDateTime;

use crate::schema::tags;

#[derive(Queryable, Serialize, Identifiable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}
