pub trait BasicClaims {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
}

impl BasicClaims for Claims {}
