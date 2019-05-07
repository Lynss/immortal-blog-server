#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub nickname: String,
    pub privileges: String,
    pub exp: usize,
    pub id:i32
}
