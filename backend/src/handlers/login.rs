use std::collections::HashMap;
use std::iter::FromIterator;

use actix_redis::{Command, RedisActor, RespValue};
use actix_web::{actix::Addr, AsyncResponder, Json, State};
use chrono::Utc;
use futures::{
    future::{self, join_all},
    Future,
};

use commons::{
    configs::{EXPIRE_TIME, PERMISSIONS_PREFIX_KEY, ROLES_PREFIX_KEY},
    utils, AppState, Claims, ImmortalError,
};

use crate::models::{
    pojos::{AuthInfo, LoginRequest, LoginResponse, Privileges},
    HandlerResponse,
};

fn store_privileges(
    redis: Addr<RedisActor>,
    roles: Vec<String>,
    permissions: Vec<(String, i32)>,
    id: i32,
) -> impl Future<Item = (), Error = ImmortalError> {
    //create task to save permissions
    let save_permissions_command: String = permissions
        .iter()
        .map(|(key, value)| vec![key.clone(), value.to_string()])
        .flatten()
        .collect::<Vec<String>>()
        .join(" ");
    let permissions_key = utils::create_prefix_key(PERMISSIONS_PREFIX_KEY, id);
    let save_permissions = redis.send(Command(resp_array![
        "HSET",
        permissions_key,
        save_permissions_command
    ]));

    //create task to save roles
    let save_roles_command = roles.join(" ");
    let roles_key = &utils::create_prefix_key(ROLES_PREFIX_KEY, id)[..];
    let save_roles = redis.send(Command(resp_array!["LPUSH", roles_key, save_roles_command]));
    //begin action
    join_all(vec![save_permissions, save_roles])
        .map_err(ImmortalError::ignore)
        .and_then(move |res| match res.as_slice() {
            [Ok(RespValue::SimpleString(x)), Ok(RespValue::SimpleString(y))]
                if x == "OK" && y == "OK" =>
            {
                Ok(())
            }
            _ => Err(ImmortalError::ignore(
                "Failed to store privileges into redis",
            )),
        })
}

pub fn login(
    (info, state): (Json<LoginRequest>, State<AppState>),
) -> HandlerResponse<LoginResponse> {
    let db = state.db.clone();
    let redis = state.redis.clone();
    db.send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .and_then(move |result| {
            match result {
                Ok(AuthInfo {
                    id,
                    roles,
                    permissions,
                }) => {
                    let expire = Utc::now().timestamp();
                    //generate token from user
                    let claims = Claims {
                        id,
                        exp: expire + EXPIRE_TIME,
                    };
                    let token = utils::jwt_encode(&claims, None);
                    store_privileges(redis, roles, permissions, id).and_then(|_| {
                        //transform vec to map structure
                        let permissions = HashMap::from_iter(permissions);
                        //get privileges of current user
                        let privileges = Privileges { roles, permissions };
                        //save privileges into redis
                        future::ok(utils::success(LoginResponse { token, privileges }))
                    })
                },
                Err(err) => future::err(err),
            }
        })
        .responder()
}
