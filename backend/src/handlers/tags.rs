use actix_web::web::Data;
use futures::future;

use common::HandlerResponse;

use crate::{utils, AppState};

pub fn get_tags(_state: Data<AppState>) -> impl HandlerResponse<i32> {
    future::done(Ok(utils::success(1)))
}

pub fn create_tag(_state: Data<AppState>) -> impl HandlerResponse<i32> {
    future::done(Ok(utils::success(1)))
}
