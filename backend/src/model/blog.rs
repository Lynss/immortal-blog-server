use serde_json::Value as JSONB;

#[derive(Queryable)]
pub struct Blog {
    id: i32,
    data: JSONB,
}
