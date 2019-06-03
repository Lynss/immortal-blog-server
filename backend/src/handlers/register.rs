use actix_web::web::{Data, Json};

use common::{HandlerResponse, ImmortalError};
use share::structs::RegisterRequest;

use crate::{utils, AppState};

pub fn register(
    (info, state): (Json<RegisterRequest>, Data<AppState>),
) -> impl HandlerResponse<()> {
    state
        .db
        .send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .and_then(|result| result.map(utils::success))
}
