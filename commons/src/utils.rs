use actix_web::Json;
use serde::Serialize;

use crate::{Immortal, ImmortalResponse, Result, Claims};

pub fn success<T: Serialize>(data: T) -> Result<Json<ImmortalResponse<T>>> {
    Ok(Json(ImmortalResponse::success(data)))
}

pub fn fail<T: Serialize>(err: Immortal) -> Result<Json<ImmortalResponse<T>>> {
    Ok(Json(ImmortalResponse::fail(err)))
}