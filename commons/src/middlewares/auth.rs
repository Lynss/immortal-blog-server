use crate::utils;
use actix_web::{
    error::ResponseError,
    middleware::{Middleware, Response, Started},
    HttpRequest, HttpResponse, Result,
};
use actix_redis::Command;

pub struct Auth;

impl Middleware<AppState> for Auth {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let req_path = req.path();
        //some paths have no need to check auth
        if req_path == "/api/login" {
            return Ok(Started::Done);
        }
        match req.headers().get("Authorization") {
            Some(header) => {
                let token = header.to_str()[7..].to_string();
                let claims = utils::jwt_decode(token, None)
                    .unwrap_or_else(|err| return Ok(Started::Response(err.error_response())));
                //get privileges through using claims
                req.redis.send(Command(resp_array!["GET", "immortal:privileges"]))
            }
            None => Ok(Started::Response(
                HttpResponse::Unauthorized()
                    .reason("There isn't a token")
                    .finish(),
            )),
        }
    }
}
