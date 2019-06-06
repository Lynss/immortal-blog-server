use actix::Handler;
use diesel::{prelude::*, sql_query, sql_types::Integer};

use common::{utils, ImmortalError, Result};

use crate::{
    structs::{AuthInfo, UserAndPrivilegesInfo, UserId},
    DBExecutor,
};

impl Handler<UserId> for DBExecutor {
    type Result = Result<UserAndPrivilegesInfo>;
    fn handle(&mut self, UserId { id }: UserId, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let query = r"select u.id,
       u.nickname,
       u.avatar,
       u.phone,
       u.created_at,
       u.updated_at,
       u.sex,
       u.email,
       array_agg(r.name)                as roles,
       array_agg(row (p.name,rp.level)) as permissions
from immortal_users u
         left join roles r on r.id = any (u.roles) and r.status = 1
         left join role_permissions rp
                   on rp.role_id = r.id
         left join permissions p on rp.permission_id = p.id
where u.id = $1
group by u.id
limit 1;
";
        let query = sql_query(query).bind::<Integer, _>(id);
        utils::log_sql(&query);
        query
            .get_result::<AuthInfo>(connection)
            .map_err(ImmortalError::ignore)
            .map(UserAndPrivilegesInfo::from)
    }
}
