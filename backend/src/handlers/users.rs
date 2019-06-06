use crate::utils;
use crate::AppState;
use actix_web::web::{Data, Query};
use common::{HandlerResponse, ImmortalError};
use share::structs::{UserConditions, UserInfo};

pub fn get_user_info_by_conditions(
    conditions: Query<UserConditions>,
    state: Data<AppState>,
) -> impl HandlerResponse<Option<UserInfo>> {
    state
        .db
        .send(conditions.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(|user_optional| utils::success(user_optional.map(|user| user.into())))
}
