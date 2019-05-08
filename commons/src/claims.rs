#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub nickname: String,
    pub id: i32,
    pub exp: i64,
}
