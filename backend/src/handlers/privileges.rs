use actix_web::{AsyncResponder, State};
use futures::future;

use commons::{AppState, utils};

use crate::models::{HandlerResponse, pojos::LoginResponse};

pub fn get_privileges(state: State<AppState>) -> HandlerResponse<LoginResponse> {
    future::done(Ok(utils::success(LoginResponse {
        token: String::from("token"),
    })))
    .responder()
}
