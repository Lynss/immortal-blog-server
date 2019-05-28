use actix_web::{
    error::ResponseError,
    middleware::{Middleware, Started}, HttpRequest, Result,
};

use common::ImmortalError;

use crate::AppState;

pub struct Auth;

impl Middleware<AppState> for Auth {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        let req = req.clone();
        let redis = req.state().redis.clone();
        let req_path = req.path();
        //some paths have no need to check auth
        if let "/api/login" | "/api/register" = req_path {
            Ok(Started::Done)
        } else if let Some(_) = req.headers().get("Authorization") {
            Ok(Started::Done)
        } else {
            Ok(Started::Response(
                ImmortalError::Unauthorized {
                    err_msg: "You may haven't logged in ",
                }
                .error_response(),
            ))
        }
    }
}
