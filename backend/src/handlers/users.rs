use crate::utils;
use crate::AppState;
use actix_web::web::{Data, HttpRequest, Json};
use common::{extractors::ComplexQuery, EmailMessage, HandlerResponse, ImmortalError};
use immortal_blog_derive::require_permissions;
use share::{
    domains::ImmortalUser,
    structs::{
        ActivatedUsers, CheckRepeatedUser, FindUserByName, FindUsers, ForbiddenUsers,
        GetAuthorOptions, SelectOption, TableRequest, TableResponse, UserAndPrivilegesInfo,
        UserConditions, UserInfo,
    },
};

#[require_permissions(user = "2")]
pub fn get_users(
    conditions: ComplexQuery<TableRequest<UserConditions>>,
    state: Data<AppState>,
) -> impl HandlerResponse<TableResponse<UserInfo>> {
    state
        .db
        .send(conditions.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}

#[require_permissions(user = "2")]
pub fn get_author_options(state: Data<AppState>) -> impl HandlerResponse<Vec<SelectOption>> {
    state
        .db
        .send(GetAuthorOptions)
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(|users| {
            utils::success(
                users
                    .iter()
                    .filter(|UserAndPrivilegesInfo(_, privileges)| {
                        utils::check_permission("blog", 3, &privileges.permissions)
                    })
                    .map(|UserAndPrivilegesInfo(info, _)| SelectOption {
                        id: info.id.to_string(),
                        name: info.nickname.clone(),
                    })
                    .collect(),
            )
        })
}

pub fn get_user_settings(
    state: Data<AppState>,
    condition: ComplexQuery<FindUserByName>,
) -> impl HandlerResponse<ImmortalUser> {
    state
        .db
        .send(condition.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}

#[require_permissions(user = "5")]
pub fn forbid_users(
    state: Data<AppState>,
    users: ComplexQuery<ForbiddenUsers>,
) -> impl HandlerResponse<usize> {
    state
        .db
        .send(users.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .map(utils::success)
}

pub fn check_repeated_user(
    conditions: ComplexQuery<UserConditions>,
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

pub fn activated_users(
    users: Json<ActivatedUsers>,
    state: Data<AppState>,
) -> impl HandlerResponse<()> {
    state
        .db
        .send(FindUsers {
            ids: users.into_inner().ids,
        })
        .map_err(ImmortalError::ignore)
        .flatten()
        .and_then(|users| {
            let content = utils::create_active_email();
            let tos = users
                .iter()
                .map(|user| (user.email.to_owned(), user.nickname.to_owned()))
                .collect();
            let message = EmailMessage {
                tos,
                content,
                from: ("ly1169134156@163.com", "Immortal Blog"),
                subject: "Activate Account",
                attachment_file: None,
            };
            utils::send_mail(message)
        })
        .map(utils::success)
}
