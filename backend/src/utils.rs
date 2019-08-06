use crate::AppState;
use actix_redis::{Command, RespValue};
use actix_web::{web::Data, FromRequest, HttpRequest};
pub use common::utils::*;
use common::{configs::USER_PREFIX_KEY, Claims, ImmortalError, Result};
use futures::{
    future::{self, join_all, IntoFuture},
    Future,
};
use redis_async::resp::FromResp;
use share::structs::{Privileges, UserAndPrivilegesInfo, UserId, UserInfo};
pub use share::utils::*;
use std::{cmp, collections::HashMap};

pub fn get_user_and_privileges_info(
    req: &HttpRequest,
    id: i32,
) -> Box<dyn Future<Item = UserAndPrivilegesInfo, Error = ImmortalError>> {
    let state = Data::<AppState>::extract(req).unwrap();
    let req = req.clone();
    Box::new(
        state
            .db
            .send(UserId { id })
            .map_err(ImmortalError::ignore)
            .and_then(move |result| {
                result.map(move |info| {
                    let UserAndPrivilegesInfo(user_info, privileges) = &info;
                    storage_user_and_privileges_info(user_info, privileges, id, &req)
                        .then(move |_| Ok(info))
                })
            })
            .flatten(),
    )
}

pub fn storage_user_and_privileges_info(
    user_info: &UserInfo,
    privileges: &Privileges,
    id: i32,
    req: &HttpRequest,
) -> impl Future<Item = (), Error = ImmortalError> {
    let state = Data::<AppState>::extract(req).unwrap();
    let redis = &state.redis;
    //save info into redis;
    let user_prefix_key = create_prefix_key(USER_PREFIX_KEY, id);
    let job_privileges = redis.clone().send(Command(resp_array![
        "HSET",
        &user_prefix_key,
        "privileges",
        serde_json::to_string(privileges).unwrap()
    ]));
    let job_user_info = redis.clone().send(Command(resp_array![
        "HSET",
        &user_prefix_key,
        "user_info",
        serde_json::to_string(user_info).unwrap()
    ]));
    let result_set = join_all(vec![job_privileges, job_user_info]);
    result_set
        .map_err(ImmortalError::ignore)
        .and_then(move |res| match res.as_slice() {
            [Ok(RespValue::Integer(x)), Ok(RespValue::Integer(y))]
                if x.clone() <= 1 && y.clone() <= 1 =>
            {
                info!("Success storage info into redis");
                Ok(())
            }
            _ => {
                error!("Redis internal server error");
                Err(ImmortalError::ignore("Redis internal server error"))
            }
        })
}

pub fn get_user_and_privileges_info_from_session(
    id: i32,
    req: &HttpRequest,
) -> impl Future<Item = UserAndPrivilegesInfo, Error = ImmortalError> {
    //try to get info from redis;
    let state = Data::<AppState>::extract(req).unwrap();
    let redis = &state.redis;
    let req = req.clone();
    let user_prefix_key = create_prefix_key(USER_PREFIX_KEY, id);
    let job_privileges =
        redis
            .clone()
            .send(Command(resp_array!["HGET", &user_prefix_key, "privileges"]));
    let job_user_info =
        redis
            .clone()
            .send(Command(resp_array!["HGET", &user_prefix_key, "user_info"]));
    join_all(vec![job_privileges, job_user_info]).then(move |res| match res {
        Ok(res) => {
            if let [Ok(privileges @ RespValue::BulkString(_)), Ok(user_info  @ RespValue::BulkString(_))] = res.as_slice() {

                let user_info =
                    serde_json::from_str(String::from_resp(user_info.clone()).unwrap().as_str())
                        .unwrap();
                let privileges =
                    serde_json::from_str(String::from_resp(privileges.clone()).unwrap().as_str())
                        .unwrap();
                Box::new(future::ok(UserAndPrivilegesInfo(user_info, privileges)))
            } else {
                get_user_and_privileges_info(&req, id)
            }
        }
        Err(e) => {
            ImmortalError::ignore(e);
            get_user_and_privileges_info(&req, id)
        }
    })
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
    jwt_decode(token, None).map(|claims: Claims| claims.id)
}

pub fn get_user_and_privileges_info_from_request(
    req: &HttpRequest,
) -> impl Future<Item = UserAndPrivilegesInfo, Error = ImmortalError> {
    get_user_id_from_header(req)
        .map(|id| get_user_and_privileges_info_from_session(id, req))
        .into_future()
        .flatten()
}

pub fn check_permissions(
    require_permissions: HashMap<String, i32>,
    req: &HttpRequest,
) -> impl Future<Item = (), Error = ImmortalError> {
    //firstly,try to get the current user info and privileges
    get_user_and_privileges_info_from_request(req).and_then(
        move |UserAndPrivilegesInfo(_, privileges)| {
            let owned_permissions = privileges.permissions;
            let has_permissions = require_permissions
                .iter()
                .any(|(name, level)| check_permission(name, level.to_owned(), &owned_permissions));
            if !has_permissions {
                Err(ImmortalError::Forbidden {
                    err_msg: "Has no permissions to access current service",
                })
            } else {
                Ok(())
            }
        },
    )
}

pub fn check_permission(name: &str, level: i32, owned_permissions: &HashMap<String, i32>) -> bool {
    let owned_permission_level = owned_permissions.get(name).unwrap_or(&0);
    let all_level = owned_permissions.get("all").unwrap_or(&0);
    let owned_permission_level = cmp::max(owned_permission_level, all_level).to_owned();
    level <= owned_permission_level
}
