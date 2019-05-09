use actix_web::{AsyncResponder, Query, State};
use futures::Future;

use commons::{utils, AppState, Immortal, ImmortalError};

use crate::models::{GetUser, HandlerResponse, ImmortalUser};

#[derive(Deserialize)]
pub struct Info {
    phone: Option<String>,
}

pub fn get_users(
    (query, state): (Query<Info>, State<AppState>),
) -> HandlerResponse<Vec<ImmortalUser>> {
    state
        .db
        .send(GetUser {
            phone: query.phone.clone(),
        })
        .map_err(ImmortalError::ignore)
        .and_then(|result| match result {
            Ok(users) => Ok(utils::success(users)),
            Err(_) => utils::fail(Immortal::InternalError("Failed to load users".into())),
        })
        .responder()
}
