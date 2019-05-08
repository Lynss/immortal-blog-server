use actix_redis::{Command, RespValue};
use actix_web::{
    error::ResponseError,
    HttpRequest,
    HttpResponse, middleware::{Middleware, Started}, Result,
};

use crate::{AppState, ImmortalError, utils};

pub struct Auth;

impl Middleware<AppState> for Auth {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        let req_path = req.path();
        //some paths have no need to check auth
        if req_path == "/api/login" {
            return Ok(Started::Done);
        }
        match req.headers().get("Authorization") {
            Some(header) => {
                let token = header.to_str().unwrap()[7..].to_string();
                let claims = match utils::jwt_decode(token, None) {
                    Ok(claims) => claims,
                    Err(err) => return Ok(Started::Response(err.error_response())),
                };
                //get privileges through using claims
                Ok(Started::Future(
                    req.state()
                        .redis
                        .send(Command(resp_array!["GET", "immortal:privileges"]))
                        .map_err(ImmortalError::ignore)
                        .and_then(|res| match res{
                            Ok(RespValue::Array()) =>
                        })
                ))
            }
            None => Ok(Started::Response(
                HttpResponse::Unauthorized()
                    .reason("There isn't a token")
                    .finish(),
            )),
        }
    }
}
