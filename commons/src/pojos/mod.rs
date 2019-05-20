use actix_web::{FutureResponse, Json};
pub use auth::*;
use commons::{ImmortalError, ImmortalResponse};

mod auth;

pub type HandlerResponse<T> = FutureResponse<Json<ImmortalResponse<T>>, ImmortalError>;
