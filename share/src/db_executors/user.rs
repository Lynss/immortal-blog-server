use actix::Handler;

use crate::{
    domains::ImmortalUser,
    schema,
    structs::{
        ActivatingUser, AuthInfo, CheckRepeatedUser, FindUserByName, FindUsers, ForbiddenUsers,
        GetAuthorOptions, OrderInfo, Pagination, TableRequest, TableResponse,
        UserAndPrivilegesInfo, UserConditions, UserInfo, UserSettingsUpdate,
    },
    DBExecutor,
};
use common::{pagination::*, utils, ImmortalError, Result};
use diesel::prelude::*;
use diesel::{sql_query, update};
use schema::immortal_users::dsl::*;

impl Handler<TableRequest<UserConditions>> for DBExecutor {
    type Result = Result<TableResponse<UserInfo>>;

    fn handle(
        &mut self,
        conditions: TableRequest<UserConditions>,
        _: &mut Self::Context,
    ) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let mut query = immortal_users.into_boxed();
        if let Some(user_conditions) = conditions.data {
            if let Some(nick) = user_conditions.nickname {
                query = query.filter(nickname.like(format!("%{}%", nick)));
            }
            if let Some(user_email) = user_conditions.email {
                query = query.filter(email.like(format!("%{}%", user_email)));
            }
            if let Some(user_roles) = user_conditions.roles {
                query = query.filter(roles.overlaps_with(user_roles));
            }
            if let Some(user_created_at) = user_conditions.created_at {
                query = query.filter(created_at.between(user_created_at.start, user_created_at.end))
            }
            if let Some(user_updated_at) = user_conditions.updated_at {
                query = query.filter(updated_at.between(user_updated_at.start, user_updated_at.end))
            }
        }
        if let Some(orders) = conditions.orders {
            for OrderInfo { field, order } in orders {
                match (field.as_str(), order.as_str()) {
                    ("updated_at", "descend") => {
                        query = query.order(updated_at.desc());
                    }
                    ("updated_at", "ascend") => {
                        query = query.order(updated_at.asc());
                    }
                    ("created_at", "descend") => {
                        query = query.order(created_at.desc());
                    }
                    ("created_at", "ascend") => {
                        query = query.order(created_at.asc());
                    }
                    _ => warn!("Invalid sort info"),
                }
            }
        }
        utils::log_sql(&query);
        let Pagination { page, per_page } = conditions.pagination;
        query
            .select((
                id, nickname, roles, email, phone, avatar, activated, sex, created_at, updated_at,
            ))
            .paginate(page)
            .per_page(per_page)
            .load_and_count_total::<UserInfo>(connection)
            .map_err(ImmortalError::ignore)
            .map(|(user_array, total)| TableResponse {
                datasource: user_array,
                total,
                per_page: per_page.to_owned(),
                page: page.to_owned(),
            })
    }
}

impl Handler<CheckRepeatedUser> for DBExecutor {
    type Result = Result<Option<ImmortalUser>>;

    fn handle(&mut self, message: CheckRepeatedUser, _: &mut Self::Context) -> Self::Result {
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

impl Handler<UserConditions> for DBExecutor {
    type Result = Result<Vec<ImmortalUser>>;

    fn handle(&mut self, user_conditions: UserConditions, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let mut query = immortal_users.into_boxed();
        if let Some(nick) = user_conditions.nickname {
            query = query.filter(nickname.like(format!("%{}%", nick)));
        }
        if let Some(user_email) = user_conditions.email {
            query = query.filter(email.like(format!("%{}%", user_email)));
        }
        if let Some(user_roles) = user_conditions.roles {
            query = query.filter(roles.overlaps_with(user_roles));
        }
        if let Some(user_created_at) = user_conditions.created_at {
            query = query.filter(created_at.between(user_created_at.start, user_created_at.end))
        }
        if let Some(user_updated_at) = user_conditions.updated_at {
            query = query.filter(updated_at.between(user_updated_at.start, user_updated_at.end))
        }
        utils::log_sql(&query);
        query
            .get_results::<ImmortalUser>(connection)
            .map_err(ImmortalError::ignore)
    }
}

impl Handler<GetAuthorOptions> for DBExecutor {
    type Result = Result<Vec<UserAndPrivilegesInfo>>;

    fn handle(&mut self, _: GetAuthorOptions, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let query = r"select u.id,
       u.nickname,
       u.avatar,
       u.phone,
       u.created_at,
       u.updated_at,
       u.sex,
       u.email,
       u.roles as role_ids,
       u.activated,
       array_agg(r.name)                as roles,
       array_agg(row (p.name,rp.level)) as permissions
from immortal_users u
         left join roles r on r.id = any (u.roles) and r.status = 1
         left join role_permissions rp
                   on rp.role_id = r.id
         left join permissions p on rp.permission_id = p.id
group by u.id
";
        let query = sql_query(query);
        utils::log_sql(&query);
        query
            .get_results::<AuthInfo>(connection)
            .map_err(ImmortalError::ignore)
            .map(|infos| {
                infos
                    .iter()
                    .cloned()
                    .map(UserAndPrivilegesInfo::from)
                    .collect()
            })
    }
}

impl Handler<ForbiddenUsers> for DBExecutor {
    type Result = Result<usize>;
    fn handle(&mut self, users: ForbiddenUsers, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = update(immortal_users.filter(id.eq_any(users.ids))).set(roles.eq(vec![1]));
        utils::log_sql(&sql);
        sql.execute(connection).map_err(ImmortalError::ignore)
    }
}

impl Handler<FindUserByName> for DBExecutor {
    type Result = Result<ImmortalUser>;
    fn handle(&mut self, condition: FindUserByName, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = immortal_users.filter(nickname.eq(condition.nickname));
        utils::log_sql(&sql);
        sql.get_result::<ImmortalUser>(connection)
            .map_err(ImmortalError::ignore)
    }
}

impl Handler<FindUsers> for DBExecutor {
    type Result = Result<Vec<ImmortalUser>>;
    fn handle(&mut self, users: FindUsers, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = immortal_users.filter(id.eq_any(users.ids));
        utils::log_sql(&sql);
        sql.get_results::<ImmortalUser>(connection)
            .map_err(ImmortalError::ignore)
    }
}

impl Handler<ActivatingUser> for DBExecutor {
    type Result = Result<usize>;
    fn handle(&mut self, user: ActivatingUser, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql =
            update(immortal_users.find(user.id)).set((activated.eq(true), roles.eq(user.roles)));
        utils::log_sql(&sql);
        sql.execute(connection).map_err(ImmortalError::ignore)
    }
}

impl Handler<UserSettingsUpdate> for DBExecutor {
    type Result = Result<usize>;
    fn handle(&mut self, message: UserSettingsUpdate, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = update(immortal_users.find(message.0)).set(message.1);
        utils::log_sql(&sql);
        sql.execute(connection).map_err(ImmortalError::ignore)
    }
}
