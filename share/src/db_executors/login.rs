use actix_web::actix::Handler;
use diesel::prelude::*;

use common::{utils, ImmortalError, Result};

use crate::{
    schema,
    structs::{LoginRequest, UserId},
    DBExecutor,
};

impl Handler<LoginRequest> for DBExecutor {
    type Result = Result<UserId>;
    fn handle(&mut self, login_request: LoginRequest, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_users::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let LoginRequest {
            nickname: ref nick,
            password: ref psd,
            remember: _,
        } = login_request;
        let query = immortal_users
            .filter(nickname.eq(nick))
            .filter(password.eq(psd))
            .select(id);
        utils::log_sql(&query);
        query
            .first::<i32>(connection)
            .map_err(|err| {
                error!("Failed to query the auth info,caused by {}", err);
                ImmortalError::Unauthorized {
                    err_msg: "Invalid nickname or password",
                }
            })
            .map(|user_id| UserId { id: user_id })
    }
}
