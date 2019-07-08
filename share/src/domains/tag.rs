use crate::schema::tags;
use chrono::NaiveDateTime;
use common::utils;

#[derive(Queryable, Serialize, Identifiable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub color: String,
    #[serde(with = "utils::date_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "utils::date_format")]
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}
