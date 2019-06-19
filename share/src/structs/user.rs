use actix::Message;

use super::TableRequest;
use crate::domains::ImmortalUser;
use common::Result;

#[derive(Deserialize, Serialize)]
pub struct UserConditions {
    pub nickname: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CheckRepeatedUser(pub String);

impl Message for CheckRepeatedUser {
    type Result = Result<Option<ImmortalUser>>;
}

impl Message for TableRequest<UserConditions> {
    type Result = Result<Vec<ImmortalUser>>;
}
