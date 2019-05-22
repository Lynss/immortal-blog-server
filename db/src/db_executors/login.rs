use actix_web::actix::Handler;
use diesel::{prelude::*, sql_query, sql_types::VarChar};

use common::{utils, AuthInfo, DBExecutor, ImmortalError, LoginRequest, Result};

impl Handler<LoginRequest> for DBExecutor {
    type Result = Result<AuthInfo>;
    fn handle(&mut self, login_request: LoginRequest, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let LoginRequest {
            nickname: ref nick,
            password: ref psd,
            remember: _,
        } = login_request;
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
where nickname = $1
  and password = $2
group by u.id
limit 1;
";
        let query = sql_query(query)
            .bind::<VarChar, _>(nick)
            .bind::<VarChar, _>(psd);
        utils::log_sql(&query);
        query.get_result::<AuthInfo>(connection).map_err(|err| {
            error!("Failed to query the auth info,caused by {}", err);
            ImmortalError::Unauthorized {
                err_msg: "Invalid nickname or password",
            }
        })
    }
}
