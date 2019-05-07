use actix_web::{AsyncResponder, Query, State};
use futures::Future;

use commons::{Immortal, ImmortalError, utils};

use crate::models::{AppState, GetUser, HandlerResponse, ImmortalUser};

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
        .map_err(|_|ImmortalError::InternalError)
        .and_then(|result| match result {
            Ok(users) => Ok(utils::success(users)),
            Err(_) => utils::fail(Immortal::InternalError("Failed to load users".into())),
        })
        .responder()
}
