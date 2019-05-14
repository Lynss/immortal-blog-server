use actix_web::actix::Handler;
use diesel::prelude::*;
use futures::Future;
use commons::{utils, DBExecutor, ImmortalError, Result,schema};
use crate::pojos::RegisterRequest;

impl Handler<RegisterRequest> for DBExecutor {
    type Result = Result<()>;
    fn handle(&mut self, info: RegisterRequest, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_users::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = insert_into(users).values(&info);
        utils::log_sql(&sql);
        sql.execute(connection).map_err(|err| {
            error!("Failed to create a new user,caused by {:?}", err);
            ImmortalError::ignore(err);
        });
        Ok(())
    }
}
