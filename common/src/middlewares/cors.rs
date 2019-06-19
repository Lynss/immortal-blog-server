use actix_http::Response;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::header::{self, HeaderValue},
    middleware::cors::AllOrSome,
    Error,
};
use futures::future::IntoFuture;
use futures::{
    future::{self, FutureResult},
    Future,
};

#[derive(Default, Clone)]
pub struct Cors {
    pub allowed_origins: AllOrSome<Vec<String>>,
}

impl Cors {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Cors {
            allowed_origins: AllOrSome::Some(allowed_origins.into()),
        }
    }
}

#[derive(Clone)]
pub struct CorsMiddleware<S> {
    service: S,
    pub allowed_origins: AllOrSome<Vec<String>>,
}

impl<S, B> Transform<S> for Cors
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CorsMiddleware<S>;
    type InitError = ();
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(CorsMiddleware {
            service,
            allowed_origins: self.allowed_origins.clone(),
        })
    }
}

impl<S, B> Service for CorsMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> futures::Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let allowed_origins = (&self.allowed_origins).clone();
        let method = req.method().clone();
        let resp_future: Box<dyn Future<Item = ServiceResponse<B>, Error = _>> =
            if method == "OPTIONS" {
                Box::new(
                    req.into_response(Response::Ok().finish().into_body())
                        .into_future(),
                )
            } else {
                Box::new(self.service.call(req))
            };
        Box::new(resp_future.and_then(|mut resp| {
            let resp_headers = resp.headers_mut();
            match allowed_origins {
                AllOrSome::All => resp_headers.insert(
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    HeaderValue::from_static("*"),
                ),
                AllOrSome::Some(origins) => resp_headers.insert(
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    HeaderValue::from_str(origins.join(",").as_str())
                        .expect("failed to transform str to header value"),
                ),
            };
            resp_headers.insert(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_static("POST,GET,OPTIONS,PUT,DELETE"),
            );
            resp_headers.insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderValue::from_static("Content-Type,Accept,Authorization"),
            );
            Ok(resp)
        }))
    }
}
