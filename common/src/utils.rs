use crate::{Claims, EmailMessage, ImmortalError, ImmortalResponse, Result};
use actix_web::web::Json;
use diesel::{debug_query, pg::Pg, query_builder::QueryFragment};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, errors::ErrorKind, Header, Validation};
use serde::Serialize;
use uuid::Uuid;

use lettre::{
    smtp::{
        authentication::{Credentials, Mechanism},
        extension::ClientId,
        ConnectionReuseParameters,
    },
    SmtpClient, Transport,
};
use lettre_email::{Email, Mailbox};

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

pub fn create_prefix_key(prefix: &str, id: i32) -> String {
    format!("{}:{}", prefix, id)
}

pub fn log_sql<T: QueryFragment<Pg>>(query: &T) {
    let debug = debug_query::<Pg, _>(&query);
    info!("Execute sql : {}", &debug);
}

pub fn ready_env() {
    dotenv().ok();
}

pub fn uuid() -> String {
    format!("{}", Uuid::new_v4())
}

pub mod date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub fn send_mail<T: Into<Mailbox>, K: Into<Mailbox>, H: Into<String>, J: Into<String>>(
    message: EmailMessage<T, K, J, H>,
) -> Result<()> {
    let mut email = Email::builder()
        .from(message.from)
        .subject(message.subject)
        .html(message.content);
    for to in message.tos {
        email = email.to(to);
    }
    if let Some(attachment) = message.attachment_file {
        email = email
            .attachment_from_file(
                attachment.path,
                attachment.filename,
                attachment.content_type,
            )
            .unwrap()
    }
    let email = email.build().unwrap();
    let mut mailer = SmtpClient::new_simple("smtp.163.com")
        .unwrap()
        .hello_name(ClientId::Domain("localhost".to_string()))
        .credentials(Credentials::new(
            String::from("ly1169134156@163.com"),
            String::from("ly19944512"),
        ))
        .smtp_utf8(true)
        .authentication_mechanism(Mechanism::Plain)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();
    // Send the email
    mailer
        .send(email.into())
        .map_err(ImmortalError::ignore)
        .and_then(|result| {
            if result.is_positive() {
                Ok(())
            } else {
                Err(ImmortalError::ignore(result.message))
            }
        })
}

pub fn create_active_email() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <p>Thank you for your confirmation!</p>
    <p>
        Click link below to activate your account<br>
        <a href='//www.baidu.com'>www.baidu.com</a>
    </p>
</body>
</html>    "#
    )
}
