use actix_web::{FutureResponse, Json};

pub use db_executor::*;
pub use immortal_response::*;
pub use immortal_user::*;
pub use state::*;

mod db_executor;
mod immortal_response;
mod immortal_user;
mod state;
//mod blog;
//pub use blog::*;

pub mod schema;

pub type HandlerResponse<T> = FutureResponse<Json<ImmortalResponse<T>>>;

//todo:forced to cast value to target type and then fill conditions with them through macro
//pub trait Conditions {}
