#![feature(trait_alias)]
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
extern crate serde;
extern crate serde_qs;
#[macro_use]
extern crate serde_derive;
extern crate actix_http;
extern crate actix_service;
extern crate futures;

pub use actix_redis::RedisActor;
use actix_web::web::Json;
use serde::Serialize;

pub use claims::*;
pub use extends::*;
use futures::Future;
pub use immortal_error::*;
pub use immortal_response::*;
use std::result;

mod claims;
pub mod configs;
mod extends;
pub mod extractors;
mod immortal_error;
mod immortal_response;
pub mod middlewares;
pub mod utils;
pub type Result<T, E = ImmortalError> = result::Result<T, E>;

pub trait HandlerResponse<T: Serialize> =
    Future<Item = Json<ImmortalResponse<T>>, Error = ImmortalError>;
