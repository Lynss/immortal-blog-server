use actix_web::{AsyncResponder, State};
use futures::future;

use commons::{utils, AppState};

use crate::models::{pojos::LoginResponse, HandlerResponse};

pub fn get_privileges(state: State<AppState>) -> HandlerResponse<LoginResponse> {
    future::done(Ok(utils::success(LoginResponse {
        token: String::from("token"),
    })))
    .responder()
}
