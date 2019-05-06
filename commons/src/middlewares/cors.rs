use actix_web::{
    http::{
        header::{self, HeaderValue},
        Method, StatusCode,
    },
    HttpRequest,
    HttpResponse, middleware::{cors::AllOrSome, Middleware, Response}, Result,
};

pub struct Cors {
    allowed_origins: AllOrSome<Vec<&'static str>>,
}

impl Default for Cors {
    fn default() -> Self {
        Cors {
            allowed_origins: AllOrSome::default(),
        }
    }
}

impl Cors {
    pub fn new(allowed_origins: Vec<&'static str>) -> Self {
        Cors {
            allowed_origins: AllOrSome::Some(allowed_origins),
        }
    }
}

//impl a simply cors middleware
impl<S> Middleware<S> for Cors {
    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        if Method::OPTIONS == *req.method() {
            //if it's a options request,return success
            resp = HttpResponse::new(StatusCode::from_u16(200).unwrap());
        };
        let resp_headers = resp.headers_mut();
        match &self.allowed_origins {
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
            HeaderValue::from_static("POST,GET,OPTIONS,DELETE"),
        );
        resp_headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("Content-Type,Accept,Authorization"),
        );
        Ok(Response::Done(resp))
    }
}
