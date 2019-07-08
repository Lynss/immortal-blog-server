#![feature(trait_alias)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub use actix_redis::RedisActor;

pub use extends::*;

mod structs;
pub use structs::*;
pub mod configs;
mod extends;
pub mod extractors;
pub mod middlewares;
pub mod utils;
