use actix_web::{AsyncResponder, State};
use futures::future;

use commons::utils;

use crate::models::{AppState, HandlerResponse, pojos::LoginResponse};

pub fn get_privileges(_state: State<AppState>) -> HandlerResponse<LoginResponse> {
    future::done(Ok(utils::success(LoginResponse {
        token: String::from("token"),
    })))
    .responder()
}
