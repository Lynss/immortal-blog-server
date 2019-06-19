use crate::utils;
use crate::AppState;
use actix_web::web::{Data, HttpRequest, Query};
use common::{HandlerResponse, ImmortalError};
use immortal_blog_derive::require_permissions;
use share::{
    domains::ImmortalUser,
    structs::{CheckRepeatedUser, TableRequest, UserConditions},
};

#[require_permissions(user = "2")]
pub fn get_user_info_by_conditions(
    conditions: Query<TableRequest<UserConditions>>,
    state: Data<AppState>,
) -> impl HandlerResponse<Vec<ImmortalUser>> {
    state
        .db
        .send(conditions.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}

pub fn check_repeated_user(
    conditions: Query<UserConditions>,
    state: Data<AppState>,
) -> impl HandlerResponse<bool> {
    let message = CheckRepeatedUser(conditions.into_inner().nickname.unwrap());
    state
        .db
        .send(message)
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(|option| utils::success(option.is_some()))
}
