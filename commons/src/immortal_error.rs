use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};

#[derive(Fail, Debug)]
pub enum ImmortalError {
    #[fail(display = "An internal error occurred.")]
    InternalError,
}

impl ResponseError for ImmortalError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ImmortalError::InternalError => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
