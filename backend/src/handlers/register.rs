use actix_web::{AsyncResponder, Json, State};
use futures::Future;

use commons::{AppState, ImmortalError, utils};

use crate::pojos::{HandlerResponse, RegisterRequest};

pub fn register(
    (info, state): (Json<RegisterRequest>, State<AppState>),
) -> HandlerResponse<()> {
    state
        .db
        .send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .and_then(|result| result.map(utils::success))
        .responder()
}
