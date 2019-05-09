use serde_json::Value as JSONB;

#[derive(Queryable, Serialize, Identifiable)]
pub struct Blog {
    id: i32,
    data: JSONB,
}
