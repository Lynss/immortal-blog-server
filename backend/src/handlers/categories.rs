use actix_web::web::{Data, HttpRequest, Json, Path};

use common::{extractors::ComplexQuery, HandlerResponse, ImmortalError};

use crate::{utils, AppState};
use futures::future::IntoFuture;
use immortal_blog_derive::require_permissions;
use share::{
    domains::Category,
    structs::{
        CategoryConditions, CategoryCreate, CategoryCreateInfo, CategoryDelete, CategoryUpdate,
        CategoryUpdateInfo, TableRequest, TableResponse, UserAndPrivilegesInfo,
    },
};

#[require_permissions(category = "2")]
pub fn get_categories(
    state: Data<AppState>,
    conditions: ComplexQuery<TableRequest<CategoryConditions>>,
) -> impl HandlerResponse<TableResponse<Category>> {
    utils::get_user_id_from_header(&req)
        .map(|id| utils::get_user_and_privileges_info_from_session(id, &req))
        .into_future()
        .flatten()
        .and_then(move |UserAndPrivilegesInfo(user_info, privileges)| {
            //Each category can only be shown to its create user unless current user has immortal role
            let mut conditions = conditions.into_inner();
            if !privileges.roles.contains(&"immortal".to_owned()) {
                let created_by = user_info.nickname;
                let mut data = conditions.data.unwrap_or_default();
                data.created_by = Some(created_by);
                conditions = TableRequest {
                    data: Some(data),
                    ..conditions
                }
            }
            state
                .db
                .send(conditions)
                .map_err(ImmortalError::ignore)
                .flatten()
                .map(utils::success)
        })
}

#[require_permissions(category = "3")]
pub fn create_category(
    state: Data<AppState>,
    category_creation_info: Json<CategoryCreateInfo>,
) -> impl HandlerResponse<()> {
    utils::get_user_id_from_header(&req)
        .map(|id| utils::get_user_and_privileges_info_from_session(id, &req))
        .into_future()
        .flatten()
        .and_then(move |UserAndPrivilegesInfo(user_info, _)| {
            state
                .db
                .send(CategoryCreate::new(
                    category_creation_info.into_inner(),
                    user_info.nickname,
                ))
                .map_err(ImmortalError::ignore)
                .flatten()
                .map(utils::success)
        })
}

#[require_permissions(category = "3")]
pub fn delete_category(
    state: Data<AppState>,
    category_delete: ComplexQuery<CategoryDelete>,
) -> impl HandlerResponse<usize> {
    state
        .db
        .send(category_delete.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}

#[require_permissions(category = "3")]
pub fn update_category(
    state: Data<AppState>,
    id: Path<i32>,
    category_update: Json<CategoryUpdateInfo>,
) -> impl HandlerResponse<()> {
    state
        .db
        .send(CategoryUpdate(
            id.into_inner(),
            category_update.into_inner(),
        ))
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}
