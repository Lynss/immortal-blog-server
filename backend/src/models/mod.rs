use actix_web::{FutureResponse, Json};

use commons::{ImmortalError, ImmortalResponse};
pub use domains::*;

mod domains;

pub mod pojos;
pub mod schema;

pub type HandlerResponse<T> = FutureResponse<Json<ImmortalResponse<T>>, ImmortalError>;

//todo:forced to cast value to target type and then fill conditions with them through macro
//pub trait Conditions {}
