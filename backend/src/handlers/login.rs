use actix_web::{AsyncResponder, Json, State};
use chrono::Utc;
use futures::Future;

use commons::{AppState, Claims, configs::EXPIRE_TIME, ImmortalError, utils};

use crate::models::{
    HandlerResponse,
    pojos::{AuthInfo, LoginRequest, LoginResponse, Privileges},
};

pub fn login(
    (info, state): (Json<LoginRequest>, State<AppState>),
) -> HandlerResponse<LoginResponse> {
    state
        .db
        .send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .and_then(|result| {
            result.map(
                |AuthInfo {
                     id,
                     roles,
                     permissions,
                 }| {
                    debug!("{},{:?},{:?}",&id, &roles, &permissions);
                    let expire = Utc::now().timestamp();
                    //generate token from user
                    let claims = Claims {
                        id,
                        exp: expire + EXPIRE_TIME,
                    };
                    let token = utils::jwt_encode(&claims, None);
                    //get privileges of current user
                    let privileges = Privileges { roles, permissions };
                    utils::success(LoginResponse { token, privileges })
                },
            )
        })
        .responder()
}
