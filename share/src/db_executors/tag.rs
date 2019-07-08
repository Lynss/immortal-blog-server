use actix::Handler;

use crate::{
    domains::Tag,
    schema,
    structs::{
        OrderInfo, Pagination, TableRequest, TableResponse, TagConditions, TagCreate, TagDelete,
        TagUpdate,
    },
    DBExecutor,
};
use common::{pagination::*, utils, ImmortalError, Result};
use diesel::{delete, insert_into, prelude::*, update};
use schema::tags::dsl::*;

impl Handler<TableRequest<TagConditions>> for DBExecutor {
    type Result = Result<TableResponse<Tag>>;

    fn handle(
        &mut self,
        conditions: TableRequest<TagConditions>,
        _: &mut Self::Context,
    ) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let mut query = tags.into_boxed();
        if let Some(tag_conditions) = conditions.data {
            if let Some(tag_name) = tag_conditions.name {
                query = query.filter(name.like(format!("%{}%", tag_name)));
            }
            if let Some(tag_created_by) = tag_conditions.created_by {
                query = query.filter(created_by.like(format!("%{}%", tag_created_by)));
            }
            if let Some(tag_created_at) = tag_conditions.created_at {
                query = query.filter(created_at.between(tag_created_at.start, tag_created_at.end))
            }
            if let Some(tag_updated_by) = tag_conditions.updated_by {
                query = query.filter(updated_by.like(format!("%{}%", tag_updated_by)));
            }
            if let Some(tag_updated_at) = tag_conditions.updated_at {
                query = query.filter(updated_at.between(tag_updated_at.start, tag_updated_at.end))
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
            .paginate(page)
            .per_page(per_page)
            .load_and_count_total::<Tag>(connection)
            .map_err(ImmortalError::ignore)
            .map(|(tag_array, total)| TableResponse {
                datasource: tag_array,
                total,
                per_page: per_page.to_owned(),
                page: page.to_owned(),
            })
    }
}

impl Handler<TagCreate> for DBExecutor {
    type Result = Result<()>;
    fn handle(&mut self, message: TagCreate, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = insert_into(tags).values(&message);
        utils::log_sql(&sql);
        sql.execute(connection)
            .map(|_| info!("Succeed to create a new tag"))
            .map_err(ImmortalError::ignore)
    }
}

impl Handler<TagDelete> for DBExecutor {
    type Result = Result<usize>;
    fn handle(&mut self, message: TagDelete, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = delete(tags.filter(id.eq_any(message.ids)));
        utils::log_sql(&sql);
        sql.execute(connection)
            .map(|num| {
                info!("Succeed to delete {} records", num);
                num
            })
            .map_err(ImmortalError::ignore)
    }
}

impl Handler<TagUpdate> for DBExecutor {
    type Result = Result<()>;
    fn handle(&mut self, message: TagUpdate, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = update(tags.find(message.0)).set(message.1);
        utils::log_sql(&sql);
        sql.execute(connection)
            .map(|_| {
                info!("Succeed to update record");
            })
            .map_err(ImmortalError::ignore)
    }
}
