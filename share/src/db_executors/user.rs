use actix::Handler;

use crate::{domains::ImmortalUser, schema, structs::UserConditions, DBExecutor};
use common::{utils, ImmortalError, Result};
use diesel::prelude::*;

impl Handler<UserConditions> for DBExecutor {
    type Result = Result<Option<ImmortalUser>>;

    fn handle(&mut self, user_conditions: UserConditions, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_users::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let mut query = immortal_users.into_boxed();
        if let Some(ref nick) = user_conditions.nickname {
            query = query.filter(nickname.eq(nick));
        }
        utils::log_sql(&query);
        query
            .first::<ImmortalUser>(connection)
            .optional()
            .map_err(ImmortalError::ignore)
    }
}
