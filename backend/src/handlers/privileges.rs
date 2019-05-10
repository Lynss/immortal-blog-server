use actix_web::{AsyncResponder, State};
use futures::future;

use commons::{AppState, utils};

use crate::models::HandlerResponse;

pub fn get_privileges(_state: State<AppState>) -> HandlerResponse<i32> {
    future::done(Ok(utils::success(1))).responder()
}
