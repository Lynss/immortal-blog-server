extern crate actix_redis;
extern crate actix_web;
extern crate chrono;
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate jsonwebtoken;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use actix_redis::RedisActor;
use actix_web::{FutureResponse, Json};
pub use dotenv::dotenv;

pub use claims::*;
pub use immortal_error::*;
pub use immortal_response::*;
use std::result;

mod claims;
pub mod configs;
mod immortal_error;
mod immortal_response;
pub mod middlewares;
pub mod utils;

pub type Result<T, E = ImmortalError> = result::Result<T, E>;

pub type HandlerResponse<T> = FutureResponse<Json<ImmortalResponse<T>>, ImmortalError>;
