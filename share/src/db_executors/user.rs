use actix::Handler;

use crate::{
    domains::ImmortalUser,
    schema,
    structs::{CheckRepeatedUser, TableRequest, UserConditions},
    DBExecutor,
};
use common::{utils, ImmortalError, Result};
use diesel::prelude::*;

impl Handler<TableRequest<UserConditions>> for DBExecutor {
    type Result = Result<Vec<ImmortalUser>>;

    fn handle(
        &mut self,
        conditions: TableRequest<UserConditions>,
        _: &mut Self::Context,
    ) -> Self::Result {
        use schema::immortal_users::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let mut query = immortal_users.into_boxed();
        if let Some(user_conditions) = conditions.data {
            if let Some(nick) = user_conditions.nickname {
                query = query.filter(nickname.eq(nick));
            }
        }
        utils::log_sql(&query);
        query
            .get_results::<ImmortalUser>(connection)
            .map_err(ImmortalError::ignore)
    }
}

impl Handler<CheckRepeatedUser> for DBExecutor {
    type Result = Result<Option<ImmortalUser>>;

    fn handle(&mut self, message: CheckRepeatedUser, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_users::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let mut query = immortal_users.into_boxed();
        query = query.filter(nickname.eq(message.0));
        utils::log_sql(&query);
        query
            .get_result::<ImmortalUser>(connection)
            .optional()
            .map_err(ImmortalError::ignore)
    }
}
