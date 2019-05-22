use serde::Serialize;

#[derive(Serialize)]
pub struct ImmortalResponse<T: Serialize> {
    pub message: String,
    pub code: i32,
    pub data: T,
}
