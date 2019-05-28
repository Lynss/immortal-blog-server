use actix_web::{AsyncResponder, State};
use futures::future;

use common::HandlerResponse;

use crate::{utils, AppState};

pub fn get_tags(_state: State<AppState>) -> HandlerResponse<i32> {
    future::done(Ok(utils::success(1))).responder()
}

pub fn create_tag(_state: State<AppState>) -> HandlerResponse<i32> {
    future::done(Ok(utils::success(1))).responder()
}
