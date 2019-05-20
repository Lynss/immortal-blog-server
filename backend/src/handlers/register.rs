use actix_web::{AsyncResponder, Json, State};
use futures::Future;

use commons::{utils, AppState, HandlerResponse, ImmortalError, RegisterRequest};

pub fn register((info, state): (Json<RegisterRequest>, State<AppState>)) -> HandlerResponse<()> {
    state
        .db
        .send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .and_then(|result| result.map(utils::success))
        .responder()
}
