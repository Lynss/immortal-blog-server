use actix_web::{AsyncResponder, Query, State};
use futures::Future;

use crate::commons::{Immortal, utils};
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
        .from_err()
        .and_then(|result| match result {
            Ok(users) => utils::success(users),
            Err(_) => utils::fail(Immortal::InternalError("Failed to load users".into())),
        })
        .responder()
}
