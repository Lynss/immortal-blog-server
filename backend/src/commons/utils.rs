use actix_web::{Json, Result};
use serde::Serialize;

use crate::commons::Immortal;
use crate::models::ImmortalResponse;

pub fn success<T: Serialize>(data: T) -> Result<Json<ImmortalResponse<T>>> {
    Ok(Json(ImmortalResponse::success(data)))
}

pub fn fail<T: Serialize>(err: Immortal) -> Result<Json<ImmortalResponse<T>>> {
    Ok(Json(ImmortalResponse::fail(err)))
}
