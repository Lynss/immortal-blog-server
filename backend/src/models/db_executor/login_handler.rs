use actix_web::{actix::Handler, error, Result};
use commons::{ImmortalError, Result};
use diesel::prelude::*;

use crate::models::{DBExecutor, ImmortalUser, pojo::LoginRequest, schema};

impl Handler<LoginRequest> for DBExecutor {
    type Result = Result<ImmortalUser>;
    fn handle(&mut self, login_request: LoginRequest, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_user::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let LoginRequest { ref nickname, ref password } = login_request;
        immortal_user
            .filter(nickname.eq(nickname))
            .filter(password.eq(password))
            .first::<ImmortalUser>(connection)
            .map_err(ImmortalError::Unauthorized { err_msg: "Invalid nickname or password" })
    }
}
