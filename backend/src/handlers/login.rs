use actix_web::{AsyncResponder, Json, State};
use futures::future;

use crate::commons::utils;
use crate::models::{
    AppState,
    HandlerResponse, pojos::{LoginRequest, LoginResponse},
};

pub fn login(
    (info, state): (Json<LoginRequest>, State<AppState>),
) -> HandlerResponse<LoginResponse> {
    future::done(utils::success(LoginResponse {
        token: String::from("token"),
    }))
    .responder()
}
