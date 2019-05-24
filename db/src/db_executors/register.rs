use actix_web::actix::Handler;
use diesel::{insert_into, prelude::*};

use common::{ImmortalError, Result, utils};

use crate::{DBExecutor, pojos::RegisterRequest, schema};

impl Handler<RegisterRequest> for DBExecutor {
    type Result = Result<()>;
    fn handle(&mut self, info: RegisterRequest, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_users::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = insert_into(immortal_users).values(&info);
        utils::log_sql(&sql);
        sql.execute(connection)
            .map(|_| info!("Succeed to create a new user"))
            .map_err(|err| {
                error!("Failed to create a new user,caused by {:?}", err);
                ImmortalError::ignore(err)
            })
    }
}
