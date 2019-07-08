use actix_web::web::{Data, HttpRequest};
use common::{HandlerResponse, ImmortalError};
use immortal_blog_derive::require_permissions;

use crate::{utils, AppState};
use share::structs::{GetRoleOptions, SelectOption};

#[require_permissions(role = "2")]
pub fn get_role_options(state: Data<AppState>) -> impl HandlerResponse<Vec<SelectOption>> {
    state
        .db
        .send(GetRoleOptions)
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(|roles| {
            utils::success(
                roles
                    .iter()
                    .map(|role| SelectOption {
                        id: role.id.to_string(),
                        name: role.name.clone(),
                    })
                    .collect(),
            )
        })
}
