use actix_redis::{Command, RespValue};
use actix_web::{
    error::ResponseError,
    middleware::{Middleware, Response, Started},
    AsyncResponder, HttpRequest, HttpResponse, Result,
};
use futures::{future::join_all, Future};
use redis_async::resp::FromResp;

use commons::{
    configs::{PERMISSIONS_PREFIX_KEY, ROLES_PREFIX_KEY},
    utils, AppState, ImmortalError, Privileges,
};
use std::collections::HashMap;

pub struct Auth;

impl Middleware<AppState> for Auth {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        let req = req.clone();
        let redis = req.state().redis;
        let req_path = req.path();
        //some paths have no need to check auth
        if let "/api/login" | "/api/register" = req_path {
            return Ok(Started::Done);
        }
        let token = match req.headers().get("Authorization") {
            Some(header) => header.to_str().unwrap()[7..].to_string(),
            None => {
                return Ok(Started::Response(
                    ImmortalError::Unauthorized {
                        err_msg: "You may haven't logged in ",
                    }
                    .error_response(),
                ));
            }
        };
        let claims = match utils::jwt_decode(token, None) {
            Ok(claims) => claims,
            Err(err) => return Ok(Started::Response(err.error_response())),
        };
        //get privileges through using claims
        let permissions_key = utils::create_prefix_key(PERMISSIONS_PREFIX_KEY, claims.id);
        let get_permissions = redis.send(Command(resp_array!["HGETALL", permissions_key]));
        let roles_key = utils::create_prefix_key(ROLES_PREFIX_KEY, claims.id);
        let get_roles = redis.send(Command(resp_array!["SMEMBERS", roles_key]));
        Ok(Started::Future(
            join_all(vec![get_permissions, get_roles])
                .from_err()
                .map(move |res| {
                    if let [Ok(permissions @ RespValue::Array(_)), Ok(roles @ RespValue::Array(_))] = res.as_slice() {
                        req.extensions_mut().insert(Identity {
                            permissions: HashMap::<String, String>::from_resp(permissions.clone())
                                .unwrap()
                                .iter()
                                .map(|(key, value)|{
                                    (key.clone(), value.parse::<i32>().unwrap())
                                })
                                .collect::<HashMap<String,i32>>(),
                            roles: Vec::<String>::from_resp(roles.clone()).unwrap(),
                        });
                        None
                    } else {
                        Some(ImmortalError::InternalError.error_response())
                    }
                })
                .responder(),
        ))
    }

    fn response(&self, req: &HttpRequest<AppState>, resp: HttpResponse) -> Result<Response> {
        //remove req extensions
        req.extensions_mut().clear();
        Ok(Response::Done(resp))
    }
}
