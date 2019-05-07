use actix_web::Json;
use serde::Serialize;

use crate::{Immortal, ImmortalResponse, Result, Claims};

pub fn success<T: Serialize>(data: T) -> Json<ImmortalResponse<T>> {
    Json(ImmortalResponse::success(data))
}

pub fn fail<T: Serialize>(err: Immortal) -> Result<Json<ImmortalResponse<T>>> {
    Ok(Json(ImmortalResponse::fail(err)))
}

pub fn encode(claims:Claims,key) {
    123
}