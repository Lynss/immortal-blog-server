use actix_web::actix::Handler;
use diesel::prelude::*;

use commons::{DBExecutor, ImmortalError, Result};

use crate::models::{ImmortalUser, pojos::LoginRequest, schema};

impl Handler<LoginRequest> for DBExecutor {
    type Result = Result<ImmortalUser>;
    fn handle(&mut self, login_request: LoginRequest, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_user::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let LoginRequest {
            nickname: ref nick,
            password: ref psd,
            remember: _,
        } = login_request;
        immortal_user
            .filter(nickname.eq(nick))
            .filter(password.eq(psd))
            .first::<ImmortalUser>(connection)
            .map_err(|_| ImmortalError::Unauthorized {
                err_msg: "Invalid nickname or password",
            })
    }
}
