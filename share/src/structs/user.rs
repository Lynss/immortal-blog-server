use actix::Message;

use crate::domains::ImmortalUser;
use common::Result;

#[derive(Deserialize, Serialize)]
pub struct UserConditions {
    pub nickname: Option<String>,
}

impl Message for UserConditions {
    type Result = Result<Option<ImmortalUser>>;
}
