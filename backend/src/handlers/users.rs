use crate::utils;
use crate::AppState;
use actix_web::web::{Data, HttpRequest, Json, Path};
use chrono::Utc;
use common::configs::ACTIVATED_EMAIL_EXPIRE_TIME;
use common::{extractors::ComplexQuery, Claims, EmailMessage, HandlerResponse, ImmortalError};
use futures::future::{join_all, IntoFuture};
use immortal_blog_derive::require_permissions;
use share::{
    domains::ImmortalUser,
    structs::{
        ActivatedUser, ActivatingUser, CheckRepeatedUser, FindUserByName, ForbiddenUsers,
        GetAuthorOptions, Messenger, SelectOption, TableRequest, TableResponse, TokenBox,
        UserAndPrivilegesInfo, UserConditions, UserInfo, UserSettingsInfo, UserSettingsUpdate,
    },
};
use std::env;

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
pub fn get_author_options(
    state: Data<AppState>,
) -> impl categoriesHandlerResponse<Vec<SelectOption>> {
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

pub fn update_user_settings(
    state: Data<AppState>,
    id: Path<i32>,
    req: HttpRequest,
    settings: Json<UserSettingsInfo>,
) -> impl HandlerResponse<UserAndPrivilegesInfo> {
    let id = id.into_inner();
    let messenger = UserSettingsUpdate(id, settings.into_inner());
    state
        .db
        .send(messenger)
        .map_err(ImmortalError::ignore)
        .flatten()
        .and_then(move |_| utils::get_user_and_privileges_info(&req, id))
        .map(utils::success)
}

#[require_permissions(user = "5")]
pub fn forbid_users(
    state: Data<AppState>,
    users: ComplexQuery<ForbiddenUsers>,
) -> impl HandlerResponse<usize> {
    let ids = users.into_inner().ids;
    let messenger = ForbiddenUsers { ids: ids.clone() };
    state
        .db
        .send(messenger)
        .map_err(ImmortalError::ignore)
        .flatten()
        .and_then(move |num| {
            join_all(
                ids.iter()
                    .map(|id| utils::get_user_and_privileges_info(&req, id.to_owned()))
                    .collect::<Vec<_>>(),
            )
            .map(move |_| utils::success(num))
        })
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

pub fn send_activated_email(
    user: Json<ActivatedUser>,
    state: Data<AppState>,
    req: HttpRequest,
) -> impl HandlerResponse<()> {
    let redis = &state.redis;
    let redis = redis.clone();
    let id = user.into_inner().id;
    utils::get_user_and_privileges_info_from_redis(id, &req)
        .and_then(move |UserAndPrivilegesInfo(user_info, _)| {
            let expire = Utc::now().timestamp();
            let claims = Claims {
                exp: expire + ACTIVATED_EMAIL_EXPIRE_TIME,
                id,
            };
            let token = utils::jwt_encode(&claims, None);
            let content = utils::create_active_email(token);
            let email_user = env::var("EMAIL").unwrap();
            let tos = vec![(user_info.email, user_info.nickname)];
            let message = EmailMessage {
                tos,
                content,
                from: (email_user, "Immortal Blog"),
                subject: "Activated Account",
                attachment_file: None,
            };
            utils::send_mail(message)
        })
        .and_then(move |_| {
            //Send message to redis
            let created_at = Utc::now().naive_local();
            let messenger = Messenger {
                message_type: "notifications".to_owned(),
                title: "Welcome to activate".to_owned(),
                content: "Thank you for your activation".to_owned(),
                href: Some("/user-center".to_owned()),
                img: Some(utils::get_assets_location("/activation.svg".to_owned())),
                created_at,
            };
            let messages = vec![(id, vec![messenger])];
            //Notify server to push message to client
            utils::produce_message(&redis, messages)
                .and_then(move |_| utils::notify_fetch_message(&redis, &vec![id]))
        })
        .map(utils::success)
}

pub fn active_user(
    token_box: Json<TokenBox>,
    state: Data<AppState>,
    req: HttpRequest,
) -> impl HandlerResponse<UserAndPrivilegesInfo> {
    let token = token_box.into_inner().token;
    utils::jwt_decode(token, None)
        .into_future()
        .and_then(move |claims: Claims| {
            let id = claims.id;
            utils::get_user_and_privileges_info_from_redis(id, &req).and_then(
                move |UserAndPrivilegesInfo(user_info, _)| {
                    let mut role = user_info.roles[0];
                    let forbidden_role = 1;
                    let target_level = 3;
                    if role < target_level && role > forbidden_role {
                        role = target_level;
                    };
                    let messenger = ActivatingUser {
                        id,
                        roles: vec![role],
                    };
                    state
                        .db
                        .send(messenger)
                        .map_err(ImmortalError::ignore)
                        .flatten()
                        .and_then(move |_| utils::get_user_and_privileges_info(&req, id))
                },
            )
        })
        .map(utils::success)
}
