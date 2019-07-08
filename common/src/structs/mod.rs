mod claims;
mod email_message;
mod immortal_error;
mod immortal_response;

use actix_web::web::Json;
pub use claims::*;
pub use email_message::*;
use futures::Future;
pub use immortal_error::*;
pub use immortal_response::*;
use serde::Serialize;
use std::result;

pub type Result<T, E = ImmortalError> = result::Result<T, E>;

pub trait HandlerResponse<T: Serialize> =
    Future<Item = Json<ImmortalResponse<T>>, Error = ImmortalError>;
