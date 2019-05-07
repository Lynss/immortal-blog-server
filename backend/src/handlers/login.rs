use crate::commons::utils;
use crate::models::{
    pojos::{LoginRequest, LoginResponse},
    AppState, HandlerResponse,
};
use actix_web::{AsyncResponder, Json, State};
use commons::{configs::EXPIRE_TIME, Claims, ImmortalError};
use futures::future;

pub fn login(
    (info, state): (Json<LoginRequest>, State<AppState>),
) -> HandlerResponse<LoginResponse> {
    state
        .db
        .send(info.into_inner)
        .map_err(ImmortalError::InternalError)
        .map(|result| {
            result.map(|user| {
                //generate token from user
                let claims = Claims {
                    nickname: user.nickname,
                    id: user.id,
                    exp: EXPIRE_TIME,
                };
            })
        })
        .responder()
}
