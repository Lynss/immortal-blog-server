use actix_web::web::{Data, HttpRequest, Json, Path};

use common::{extractors::ComplexQuery, HandlerResponse, ImmortalError};

use crate::{utils, AppState};
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
    state
        .db
        .send(conditions.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}

#[require_permissions(category = "3")]
pub fn create_category(
    state: Data<AppState>,
    category_creation_info: Json<CategoryCreateInfo>,
) -> impl HandlerResponse<()> {
    utils::get_user_and_privileges_info_from_request(&req).and_then(
        move |UserAndPrivilegesInfo(user_info, _)| {
            state
                .db
                .send(CategoryCreate::new(
                    category_creation_info.into_inner(),
                    user_info.nickname,
                ))
                .map_err(ImmortalError::ignore)
                .flatten()
                .map(utils::success)
        },
    )
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
