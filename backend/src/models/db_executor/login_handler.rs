use actix_web::actix::Handler;
use diesel::{debug_query, pg::Pg, prelude::*, sql_query, sql_types::VarChar};
use diesel::query_dsl::load_dsl::LoadQuery;

use commons::{DBExecutor, ImmortalError, Result};

use crate::models::{
    pojos::{AuthInfo, LoginRequest},
    schema,
};

impl Handler<LoginRequest> for DBExecutor {
    type Result = Result<AuthInfo>;
    fn handle(&mut self, login_request: LoginRequest, _: &mut Self::Context) -> Self::Result {
        use schema::immortal_users::dsl::*;
        let connection: &PgConnection = &self.0.get().unwrap();
        let LoginRequest {
            nickname: ref nick,
            password: ref psd,
            remember: _,
        } = login_request;
        let query = r"select u.id, array_agg(r.name) as roles, array_agg(row (p.name,rp.level)) as permissions
from immortal_users u
         left join roles r on r.id = any (u.roles) and r.status = 1
         left join role_permissions rp
                   on rp.role_id = r.id
         left join permissions p on rp.permission_id = p.id
where nickname = $1
  and password = $2
group by u.id limit 1";
        let query = sql_query(query);
        let debug = debug_query::<Pg, _>(&query);
        info!("ready execute query : {:?}", &debug);
        query
            .bind::<VarChar, _>(nick)
            .bind::<VarChar, _>(psd)
            .get_result::<AuthInfo>(connection)
            .map_err(|err| {
                error!("failed to query the auth info {}",err);
                ImmortalError::Unauthorized {
                    err_msg: "Invalid nickname or password",
                }
            })
    }
}
