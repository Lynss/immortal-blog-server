extern crate actix_redis;
extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate failure;
extern crate jsonwebtoken;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use actix_redis::RedisActor;
use actix_web::{FutureResponse, Json};
pub use dotenv::dotenv;

pub use claims::*;
pub use db::*;
pub use domains::*;
pub use identity::*;
pub use immortal_error::*;
pub use immortal_response::*;
pub use state::*;
use std::result;

mod claims;
pub mod configs;
mod db;
mod domains;
mod identity;
mod immortal_error;
mod immortal_response;
pub mod middlewares;
mod state;
pub mod utils;
pub mod schema;

pub type Result<T, E = ImmortalError> = result::Result<T, E>;

pub type HandlerResponse<T> = FutureResponse<Json<ImmortalResponse<T>>, ImmortalError>;
