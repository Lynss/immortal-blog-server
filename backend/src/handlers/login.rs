use actix_web::{AsyncResponder, Json, State};
use chrono::Utc;
use futures::Future;

use commons::{configs::EXPIRE_TIME, utils, AppState, Claims, ImmortalError};

use crate::models::{
    pojos::{LoginRequest, LoginResponse},
    HandlerResponse,
};

pub fn login(
    (info, state): (Json<LoginRequest>, State<AppState>),
) -> HandlerResponse<LoginResponse> {
    state
        .db
        .send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .and_then(|result| {
            result.map(|user| {
                let expire = Utc::now().timestamp();
                //generate token from user
                let claims = Claims {
                    id: user.id,
                    exp: expire + EXPIRE_TIME,
                };
                let token = utils::jwt_encode(&claims, None);
                //get privileges of current user
                utils::success(LoginResponse { token })
            })
        })
        .responder()
}
