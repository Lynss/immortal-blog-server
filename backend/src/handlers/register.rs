use actix_web::{AsyncResponder, State, Json};

use commons::{utils, AppState, ImmortalError};

use crate::models::{pojos::RegisterRequest, HandlerResponse};

pub fn register(
    (info, state): (Json<RegisterRequest>, State<AppState>),
) -> HandlerResponse<()> {
    state
        .db
        .send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .and_then(|result| result.map(||utils::success(())))
        .responder()
}
