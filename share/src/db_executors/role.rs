use actix::Handler;
use diesel::prelude::*;

use common::{utils, ImmortalError, Result};

use crate::{domains::Role, schema, structs::GetRoleOptions, DBExecutor};

impl Handler<GetRoleOptions> for DBExecutor {
    type Result = Result<Vec<Role>>;
    fn handle(&mut self, _: GetRoleOptions, _: &mut Self::Context) -> Self::Result {
        use schema::roles::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = roles.filter(status.eq(1));
        utils::log_sql(&sql);
        sql.get_results(connection).map_err(ImmortalError::ignore)
    }
}
