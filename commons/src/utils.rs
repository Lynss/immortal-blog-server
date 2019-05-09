use actix_web::Json;
use jsonwebtoken::{decode, encode, errors::ErrorKind, Header, Validation};
use serde::Serialize;

use crate::configs::PRIVILEGES_PREFIX_KEY;
use crate::{Claims, Immortal, ImmortalError, ImmortalResponse, Result};

pub fn success<T: Serialize>(data: T) -> Json<ImmortalResponse<T>> {
    Json(ImmortalResponse::success(data))
}

pub fn fail<T: Serialize>(err: Immortal) -> Result<Json<ImmortalResponse<T>>> {
    Ok(Json(ImmortalResponse::fail(err)))
}

const KEY: &'static str = "secret";

pub fn jwt_encode(claims: &Claims, header: Option<Header>) -> String {
    encode(&header.unwrap_or_default(), claims, KEY.as_ref()).unwrap()
}

pub fn jwt_decode(token: String, validation: Option<Validation>) -> Result<Claims, ImmortalError> {
    let validation = validation.unwrap_or(Validation {
        leeway: 60,
        ..Default::default()
    });
    match decode::<Claims>(&token, KEY.as_ref(), &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err(ImmortalError::Unauthorized {
                err_msg: "Token had expired",
            }),
            _ => Err(ImmortalError::Unauthorized {
                err_msg: "Invalid token",
            }),
        },
    }
}

pub fn create_privileges_key(info: i32) -> String {
    format!("{}_{}", &PRIVILEGES_PREFIX_KEY, info)
}
