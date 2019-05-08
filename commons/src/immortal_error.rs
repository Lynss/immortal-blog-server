use std::error::Error;

use actix_web::{dev::HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};

#[derive(Fail, Debug)]
pub enum ImmortalError {
    #[fail(display = "An internal error occurred.")]
    InternalError,
    #[fail(display = "Unauthorized.{}", err_msg)]
    Unauthorized { err_msg: &'static str },
    #[fail(display = "Forbidden.{}", err_msg)]
    Forbidden { err_msg: &'static str },
}

impl ImmortalError {
    pub fn ignore<T>(any: T) -> Self {
        ImmortalError::InternalError
    }
}

impl ResponseError for ImmortalError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ImmortalError::InternalError => {
                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish()
            }
            ImmortalError::Unauthorized { err_msg } => {
                HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
                    .reason(err_msg)
                    .finish()
            }
            ImmortalError::Forbidden { err_msg } => HttpResponseBuilder::new(StatusCode::FORBIDDEN)
                .reason(err_msg)
                .finish(),
        }
    }
}
