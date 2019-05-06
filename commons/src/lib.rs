extern crate actix_web;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub use immortal_error::*;
pub use immortal_response::*;
use std::result;

pub mod configs;
mod immortal_error;
mod immortal_response;
pub mod middlewares;
pub mod utils;

pub enum Immortal {
    Success,
    InternalError(String),
}

struct CodeMessage {
    pub code: i32,
    pub message: String,
}

impl Immortal {
    fn value(&self) -> CodeMessage {
        match *self {
            Immortal::Success => CodeMessage {
                code: 200,
                message: "Request success".into(),
            },
            Immortal::InternalError(ref err_cause) => CodeMessage {
                code: 500,
                message: format!("Internal server error caused by {}", err_cause),
            },
        }
    }
    pub fn code(&self) -> i32 {
        self.value().code
    }
    pub fn message(&self) -> String {
        self.value().message
    }
}

pub type Result<T, E = ImmortalError> = result::Result<T, E>;
