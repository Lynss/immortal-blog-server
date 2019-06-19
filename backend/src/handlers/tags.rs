use actix_web::web::{Data, HttpRequest, Json, Path};

use common::{extractors::ComplexQuery, HandlerResponse, ImmortalError};

use crate::{utils, AppState};
use futures::future::IntoFuture;
use immortal_blog_derive::require_permissions;
use share::{
    domains::Tag,
    structs::{
        TableRequest, TableResponse, TagConditions, TagCreate, TagCreateInfo, TagDelete, TagUpdate,
        TagUpdateInfo, UserAndPrivilegesInfo,
    },
};

#[require_permissions(tag = "2")]
pub fn get_tags(
    state: Data<AppState>,
    conditions: ComplexQuery<TableRequest<TagConditions>>,
) -> impl HandlerResponse<TableResponse<Tag>> {
    utils::get_user_id_from_header(&req)
        .map(|id| utils::get_user_and_privileges_info_from_session(id, &req))
        .into_future()
        .flatten()
        .and_then(move |UserAndPrivilegesInfo(user_info, privileges)| {
            //Each tag can only be shown to its create user unless current user has immortal role
            let mut conditions = conditions.into_inner();
            if !privileges.roles.contains(&"immortal".to_owned()) {
                let created_by = user_info.nickname;
                let mut data = conditions.data.unwrap_or(TagConditions::default());
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

#[require_permissions(tag = "3")]
pub fn create_tag(
    state: Data<AppState>,
    tag_creation_info: Json<TagCreateInfo>,
) -> impl HandlerResponse<()> {
    utils::get_user_id_from_header(&req)
        .map(|id| utils::get_user_and_privileges_info_from_session(id, &req))
        .into_future()
        .flatten()
        .and_then(move |UserAndPrivilegesInfo(user_info, _)| {
            state
                .db
                .send(TagCreate::new(
                    tag_creation_info.into_inner(),
                    user_info.nickname,
                ))
                .map_err(ImmortalError::ignore)
                .flatten()
                .map(utils::success)
        })
}

#[require_permissions(tag = "3")]
pub fn delete_tag(
    state: Data<AppState>,
    tag_delete: ComplexQuery<TagDelete>,
) -> impl HandlerResponse<usize> {
    state
        .db
        .send(tag_delete.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}

#[require_permissions(tag = "3")]
pub fn update_tag(
    state: Data<AppState>,
    id: Path<i32>,
    tag_update: Json<TagUpdateInfo>,
) -> impl HandlerResponse<()> {
    println!("{}", id.clone());
    state
        .db
        .send(TagUpdate(id.into_inner(), tag_update.into_inner()))
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}
