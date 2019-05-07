use actix_web::{error::ResponseError, http::StatusCode, HttpResponse, HttpResponseBuilder};

#[derive(Fail, Debug)]
pub enum ImmortalError {
    #[fail(display = "An internal error occurred.")]
    InternalError,
    #[fail(display = "Unauthorized.", err_msg)]
    Unauthorized { err_msg: String },
}

impl ResponseError for ImmortalError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ImmortalError::InternalError => {
                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish()
            }
            ImmortalError::Unauthorized { err_msg } => {
                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR)
                    .reason(err_msg)
                    .finish()
            }
        }
    }
}
