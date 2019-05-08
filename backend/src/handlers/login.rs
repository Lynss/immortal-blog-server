use actix_web::{AsyncResponder, Json, State};
use chrono::Utc;
use futures::Future;

use commons::{AppState, Claims, configs::EXPIRE_TIME, ImmortalError, utils};

use crate::models::{
    HandlerResponse,
    pojos::{LoginRequest, LoginResponse},
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
                    nickname: user.nickname,
                    id: user.id,
                    exp: expire + EXPIRE_TIME,
                };
                let token = utils::jwt_encode(&claims, None);
                utils::success(LoginResponse { token })
            })
        })
        .responder()
}
