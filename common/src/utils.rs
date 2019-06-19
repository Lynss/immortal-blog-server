use actix_web::web::Json;
use chrono::NaiveDateTime;
use diesel::{debug_query, pg::Pg, query_builder::QueryFragment};
use jsonwebtoken::{decode, encode, errors::ErrorKind, Header, Validation};
use serde::{Serialize, Serializer};

use crate::{Claims, ImmortalError, ImmortalResponse, Result};
use dotenv::dotenv;

pub fn success<T: Serialize>(data: T) -> Json<ImmortalResponse<T>> {
    Json(ImmortalResponse {
        code: 200,
        data,
        message: "".to_owned(),
    })
}

const KEY: &'static str = "secret";

pub fn jwt_encode(claims: &Claims, header: Option<Header>) -> String {
    encode(&header.unwrap_or_default(), claims, KEY.as_ref()).unwrap()
}

pub fn jwt_decode(token: String, validation: Option<Validation>) -> Result<Claims, ImmortalError> {
    let validation = validation.unwrap_or(Validation {
        leeway: 60,
        ..Default::default()
    });
    match decode::<Claims>(&token, KEY.as_ref(), &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err(ImmortalError::Unauthorized {
                err_msg: "Token had expired",
            }),
            _ => Err(ImmortalError::Unauthorized {
                err_msg: "Invalid token",
            }),
        },
    }
}

pub fn create_prefix_key(prefix: &str, info: i32) -> String {
    format!("{}:{}", prefix, info)
}

pub fn log_sql<T: QueryFragment<Pg>>(query: &T) {
    let debug = debug_query::<Pg, _>(&query);
    info!("Execute sql : {}", &debug);
}

pub fn ready_env() {
    use std::env;
    dotenv().ok();
    env::vars().for_each(|(key, value)| debug!("{}:{}", key, value));
}

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub fn format_time<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}
