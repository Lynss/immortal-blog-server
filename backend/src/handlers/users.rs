use crate::utils;
use crate::AppState;
use actix_web::web::{Data, HttpRequest, Json};
use chrono::Utc;
use common::configs::ACTIVATED_EMAIL_EXPIRE_TIME;
use common::{
    extractors::ComplexQuery, ActivatedEmailClaims, EmailMessage, HandlerResponse, ImmortalError,
};
use immortal_blog_derive::require_permissions;
use share::{
    domains::ImmortalUser,
    structs::{
        ActivatedUsers, CheckRepeatedUser, FindUserByName, FindUsers, ForbiddenUsers,
        GetAuthorOptions, Messenger, SelectOption, TableRequest, TableResponse,
        UserAndPrivilegesInfo, UserConditions, UserInfo,
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
    let redis = &state.redis;
    let redis = redis.clone();
    let ids = users.into_inner().ids;
    let notify_ids = ids.clone();
    state
        .db
        .send(FindUsers { ids: ids.clone() })
        .map_err(ImmortalError::ignore)
        .flatten()
        .and_then(move |users| {
            let expire = Utc::now().timestamp();
            let claims = ActivatedEmailClaims {
                exp: expire + ACTIVATED_EMAIL_EXPIRE_TIME,
                ids,
            };
            let token = utils::jwt_encode(&claims, None);
            let content = utils::create_active_email(token);
            let email_user = env::var("EMAIL").unwrap();
            let tos = users
                .iter()
                .map(|user| (user.email.to_owned(), user.nickname.to_owned()))
                .collect();
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
            let messages = notify_ids
                .iter()
                .map(|id| {
                    let created_at = Utc::now().naive_local();
                    let messenger = Messenger {
                        message_type: "notifications".to_owned(),
                        title: "Welcome to activate".to_owned(),
                        content: "Thank you for your activity".to_owned(),
                        href: None,
                        img: Some(utils::get_assets_location("/activation.svg".to_owned())),
                        created_at,
                    };
                    (id.to_owned(), vec![messenger])
                })
                .collect();
            //Notify server to push message to client
            utils::produce_message(&redis, messages)
                .and_then(move |_| utils::notify_fetch_message(&redis, &notify_ids))
        })
        .map(utils::success)
}
