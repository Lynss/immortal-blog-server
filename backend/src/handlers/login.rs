use std::collections::HashMap;
use std::iter::FromIterator;

use actix_redis::{Command, RedisActor, RespValue};
use actix_web::{actix::Addr, AsyncResponder, Json, State};
use chrono::Utc;
use futures::{future::join_all, Future, IntoFuture};

use common::{
    configs::{EXPIRE_TIME, PERMISSIONS_PREFIX_KEY, ROLES_PREFIX_KEY},
    utils, AppState, AuthInfo, Claims, HandlerResponse, ImmortalError, LoginRequest, LoginResponse,
    Privileges,
};

fn store_privileges(
    redis: Addr<RedisActor>,
    roles: &Vec<String>,
    permissions: &Vec<(String, i32)>,
    id: i32,
) -> impl Future<Item = (), Error = ImmortalError> {
    //create task to save permissions
    let mut save_permissions_command = permissions
        .iter()
        .map(|(key, value)| vec![key.clone(), value.to_string()])
        .flatten()
        .collect::<Vec<String>>();
    let permissions_key = utils::create_prefix_key(PERMISSIONS_PREFIX_KEY, id);
    let save_permissions = redis.send(Command(
        resp_array!["HSET", permissions_key].append(&mut save_permissions_command),
    ));
    let roles_key = &utils::create_prefix_key(ROLES_PREFIX_KEY, id)[..];
    let save_roles = redis.send(Command(
        resp_array!["SADD", roles_key].append(&mut roles.clone()),
    ));
    //begin action
    join_all(vec![save_permissions, save_roles])
        .map_err(ImmortalError::ignore)
        .and_then(move |res| match res.as_slice() {
            [Ok(RespValue::Integer(x)), Ok(RespValue::Integer(y))]
                if x.clone() >= 0 && y.clone() >= 0 =>
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
            result
                .map(
                    |AuthInfo {
                         id,
                         email,
                         nickname,
                         phone,
                         avatar,
                         created_at,
                         updated_at,
                         sex,
                         roles,
                         permissions,
                     }| {
                        let expire = Utc::now().timestamp();
                        //generate token from user
                        let claims = Claims {
                            id,
                            exp: expire + EXPIRE_TIME,
                        };
                        let token = utils::jwt_encode(&claims, None);
                        //save privileges into redis
                        store_privileges(redis, &roles, &permissions, id).and_then(|_| {
                            //transform vec to map structure
                            let permissions = HashMap::from_iter(permissions);
                            //get privileges of current user
                            let privileges = Privileges { roles, permissions };
                            Ok(utils::success(LoginResponse { token, privileges }))
                        })
                    },
                )
                .into_future()
                .flatten()
        })
        .responder()
}
