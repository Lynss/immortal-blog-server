use crate::AppState;
use actix_session::Session;
use actix_web::{web::Data, FromRequest, HttpRequest};
pub use common::utils::*;
use common::{
    configs::{PRIVILEGES_PREFIX_KEY, USER_INFO_PREFIX_KEY},
    ImmortalError, Result,
};
use futures::future::IntoFuture;
use futures::{future, Future};
use share::structs::{Privileges, UserAndPrivilegesInfo, UserId, UserInfo};
use std::{cmp, collections::HashMap};

pub fn get_user_and_privileges_info(
    req: &HttpRequest,
    id: i32,
) -> impl Future<Item = UserAndPrivilegesInfo, Error = ImmortalError> {
    let state = Data::<AppState>::extract(req).unwrap();
    let session = Session::extract(req).unwrap();
    state
        .db
        .send(UserId { id })
        .map_err(ImmortalError::ignore)
        .and_then(move |result| {
            result.map(move |info| {
                let UserAndPrivilegesInfo(user_info, privileges) = &info;
                storage_user_and_privileges_info(user_info, privileges, id, session);
                info
            })
        })
}

pub fn storage_user_and_privileges_info(
    user_info: &UserInfo,
    privileges: &Privileges,
    id: i32,
    session: Session,
) {
    //save info into session;
    let privileges_prefix_key = create_prefix_key(PRIVILEGES_PREFIX_KEY, id);
    let user_info_prefix_key = create_prefix_key(USER_INFO_PREFIX_KEY, id);
    session
        .set(privileges_prefix_key.as_str(), privileges)
        .unwrap();
    session
        .set(user_info_prefix_key.as_str(), user_info)
        .unwrap();
}

pub fn get_user_and_privileges_info_from_session(
    id: i32,
    req: &HttpRequest,
) -> Box<dyn Future<Item = UserAndPrivilegesInfo, Error = ImmortalError>> {
    //try to get info from session;
    let session = Session::extract(req).unwrap();
    let privileges_prefix_key = create_prefix_key(PRIVILEGES_PREFIX_KEY, id);
    let user_info_prefix_key = create_prefix_key(USER_INFO_PREFIX_KEY, id);
    let privileges = session.get(privileges_prefix_key.as_str()).unwrap();
    let user_info = session.get(user_info_prefix_key.as_str()).unwrap();
    if privileges.is_some() && user_info.is_some() {
        Box::new(future::ok(UserAndPrivilegesInfo(
            user_info.unwrap(),
            privileges.unwrap(),
        )))
    } else {
        Box::new(get_user_and_privileges_info(req, id))
    }
}

pub fn get_user_id_from_header(req: &HttpRequest) -> Result<i32> {
    let token = match req.headers().get("Authorization") {
        Some(header) => header.to_str().unwrap()[7..].to_string(),
        None => {
            return Err(ImmortalError::Unauthorized {
                err_msg: "You may haven't logged in ",
            });
        }
    };
    jwt_decode(token, None).map(|claims| claims.id)
}

pub fn check_permissions(
    require_permissions: HashMap<String, i32>,
    req: &HttpRequest,
) -> impl Future<Item = (), Error = ImmortalError> {
    //firstly,try to get the current user info and privileges
    get_user_id_from_header(&req)
        .map(|id| get_user_and_privileges_info_from_session(id, req))
        .into_future()
        .flatten()
        .and_then(move |UserAndPrivilegesInfo(_, privileges)| {
            let owned_permissions = privileges.permissions;
            let has_permissions = require_permissions.iter().any(|(name, level)| {
                let owned_permission_level = owned_permissions.get(name).unwrap_or(&0);
                let all_level = owned_permissions.get("all").unwrap_or(&0);
                let owned_permission_level = cmp::max(owned_permission_level, all_level);
                level <= owned_permission_level
            });
            if !has_permissions {
                Err(ImmortalError::Forbidden {
                    err_msg: "Has no permissions to access current service",
                })
            } else {
                Ok(())
            }
        })
}
