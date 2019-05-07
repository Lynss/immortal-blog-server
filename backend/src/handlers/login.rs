use actix_web::{AsyncResponder, Json, State};
use futures::Future;

use commons::{Claims, configs::EXPIRE_TIME, ImmortalError, utils};

use crate::models::{
    AppState,
    HandlerResponse, pojos::{LoginRequest, LoginResponse},
};

pub fn login(
    (info, state): (Json<LoginRequest>, State<AppState>),
) -> HandlerResponse<LoginResponse> {
    state
        .db
        .send(info.into_inner())
        .map_err(|_|ImmortalError::InternalError)
        .and_then(|result| {
            result.map(|user| {
                //generate token from user
                let claims = Claims {
                    nickname: user.nickname,
                    id: user.id,
                    exp: EXPIRE_TIME,
                };
                let token = utils::jwt_encode(&claims, None);
                utils::success(LoginResponse { token })
            })
        })
        .responder()
}
