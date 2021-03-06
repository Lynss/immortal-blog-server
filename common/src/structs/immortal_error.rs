use actix_web::{dev::HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};
use std::fmt::Debug;

#[derive(Fail, Debug)]
pub enum ImmortalError {
    #[fail(display = "An internal error occurred.")]
    InternalError,
    #[fail(display = "Unauthorized.{}", err_msg)]
    Unauthorized { err_msg: &'static str },
    #[fail(display = "Forbidden.{}", err_msg)]
    Forbidden { err_msg: &'static str },
    #[fail(display = "Bad request.{}", err_msg)]
    BadRequest { err_msg: &'static str },
}

impl ImmortalError {
    pub fn ignore<T: Debug>(err: T) -> Self {
        error!("Error caused by {:#?}", err);
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
            ImmortalError::BadRequest { err_msg } => {
                HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                    .reason(err_msg)
                    .finish()
            }
        }
    }
}
