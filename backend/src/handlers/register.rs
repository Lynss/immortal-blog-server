use actix_web::{AsyncResponder, Json, State};
use futures::Future;

use common::{HandlerResponse, ImmortalError};
use share::structs::RegisterRequest;

use crate::{utils, AppState};

pub fn register((info, state): (Json<RegisterRequest>, State<AppState>)) -> HandlerResponse<()> {
    state
        .db
        .send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .and_then(|result| result.map(utils::success))
        .responder()
}
