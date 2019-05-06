#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    nickname: String,
    privileges: String,
    exp: usize,
}
