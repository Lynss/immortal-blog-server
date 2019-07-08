use crate::schema::categories;
use chrono::NaiveDateTime;
use common::utils;

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "utils::date_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "utils::date_format")]
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}
