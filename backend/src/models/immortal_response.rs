use serde::Serialize;

use crate::commons::Immortal;

#[derive(Serialize)]
pub struct ImmortalResponse<T: Serialize> {
    message: String,
    code: i32,
    data: Option<T>,
}

impl<T: Serialize> ImmortalResponse<T> {
    pub fn from_immortal(immortal: Immortal, data: Option<T>) -> Self {
        ImmortalResponse {
            data,
            code: immortal.code(),
            message: immortal.message(),
        }
    }

    pub fn success(data: T) -> Self {
        ImmortalResponse::from_immortal(Immortal::Success, Some(data))
    }

    pub fn fail(err: Immortal) -> Self {
        ImmortalResponse::from_immortal(err, None)
    }
}
