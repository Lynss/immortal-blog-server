use actix_web::{dev::Payload, error::ErrorBadRequest, Error, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use serde_qs;
use std::{fmt, ops};

pub struct ComplexQuery<T>(T);

impl<T> ComplexQuery<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for ComplexQuery<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: DeserializeOwned> FromRequest for ComplexQuery<T> {
    type Error = Error;
    type Future = Result<Self, Self::Error>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        serde_qs::from_str::<T>(req.query_string())
            .map(|inner| ComplexQuery(inner))
            .map_err(ErrorBadRequest)
    }
}

impl<T> ops::DerefMut for ComplexQuery<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Debug> fmt::Debug for ComplexQuery<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for ComplexQuery<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
