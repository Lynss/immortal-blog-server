use actix_web::{
    web::{Data, Json},
    HttpRequest,
};
use chrono::Utc;
use common::{configs::EXPIRE_TIME, Claims, HandlerResponse, ImmortalError};
use share::structs::{LoginRequest, LoginResponse, UserAndPrivilegesInfo, UserId};

use crate::{utils, AppState};

pub fn login(
    info: Json<LoginRequest>,
    state: Data<AppState>,
    req: HttpRequest,
) -> impl HandlerResponse<LoginResponse> {
    state
        .db
        .send(info.into_inner())
        .map_err(ImmortalError::ignore)
        .flatten()
        .and_then(move |UserId { id }| {
            let expire = Utc::now().timestamp();
            //generate token from user
            let claims = Claims {
                id,
                exp: expire + EXPIRE_TIME,
            };
            let token = utils::jwt_encode(&claims, None);
            utils::get_user_and_privileges_info(&req, id).map(
                move |UserAndPrivilegesInfo(user_info, privileges)| {
                    utils::success(LoginResponse {
                        token,
                        privileges,
                        user_info,
                    })
                },
            )
        })
}
