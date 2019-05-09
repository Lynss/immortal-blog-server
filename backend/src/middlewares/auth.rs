use std::collections::HashMap;

use actix_redis::{Command, RespValue};
use actix_web::{
    error::ResponseError,
    middleware::{Middleware, Response, Started},
    AsyncResponder, HttpRequest, HttpResponse, Result,
};
use futures::Future;
use redis_async::resp::FromResp;

use commons::{utils, AppState, Identity, ImmortalError};

pub struct Auth;

impl Middleware<AppState> for Auth {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        let req = req.clone();
        let req_path = req.path();
        //some paths have no need to check auth
        if req_path == "/api/login" {
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
                ))
            }
        };
        let claims = match utils::jwt_decode(token, None) {
            Ok(claims) => claims,
            Err(err) => return Ok(Started::Response(err.error_response())),
        };
        //get privileges through using claims
        let key = utils::create_privileges_key(claims.id);
        Ok(Started::Future(
            req.state()
                .redis
                .send(Command(resp_array!["LRANGE", key, "0", "-1"]))
                .from_err()
                .map(move |res| match res {
                    Ok(RespValue::Nil) => None,
                    Ok(privilege_array @ RespValue::Array(_)) => {
                        req.extensions_mut().insert(Identity(Box::new(
                            HashMap::<String, i32>::from_resp(privilege_array).unwrap(),
                        )));
                        None
                    }
                    _ => Some(ImmortalError::InternalError.error_response()),
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
