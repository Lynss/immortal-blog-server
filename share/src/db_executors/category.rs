use actix::Handler;

use crate::{
    domains::Category,
    schema,
    structs::{
        CategoryConditions, CategoryCreate, CategoryDelete, CategoryUpdate, OrderInfo, Pagination,
        TableRequest, TableResponse,
    },
    DBExecutor,
};
use common::{pagination::*, utils, ImmortalError, Result};
use diesel::{delete, insert_into, prelude::*, update};
use schema::categories::dsl::*;

impl Handler<TableRequest<CategoryConditions>> for DBExecutor {
    type Result = Result<TableResponse<Category>>;

    fn handle(
        &mut self,
        conditions: TableRequest<CategoryConditions>,
        _: &mut Self::Context,
    ) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let mut query = categories.into_boxed();
        if let Some(category_conditions) = conditions.data {
            if let Some(category_name) = category_conditions.name {
                query = query.filter(name.like(format!("%{}%", category_name)));
            }
            if let Some(category_created_by) = category_conditions.created_by {
                query = query.filter(created_by.like(format!("%{}%", category_created_by)));
            }
            if let Some(category_created_at) = category_conditions.created_at {
                query = query
                    .filter(created_at.between(category_created_at.start, category_created_at.end))
            }
            if let Some(category_updated_by) = category_conditions.updated_by {
                query = query.filter(updated_by.like(format!("%{}%", category_updated_by)));
            }
            if let Some(category_updated_at) = category_conditions.updated_at {
                query = query
                    .filter(updated_at.between(category_updated_at.start, category_updated_at.end))
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
            .load_and_count_total::<Category>(connection)
            .map_err(ImmortalError::ignore)
            .map(|(category_array, total)| TableResponse {
                datasource: category_array,
                total,
                per_page: per_page.to_owned(),
                page: page.to_owned(),
            })
    }
}

impl Handler<CategoryCreate> for DBExecutor {
    type Result = Result<()>;
    fn handle(&mut self, message: CategoryCreate, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = insert_into(categories).values(&message);
        utils::log_sql(&sql);
        sql.execute(connection)
            .map(|_| info!("Succeed to create a new category"))
            .map_err(ImmortalError::ignore)
    }
}

impl Handler<CategoryDelete> for DBExecutor {
    type Result = Result<usize>;
    fn handle(&mut self, message: CategoryDelete, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = delete(categories.filter(id.eq_any(message.ids)));
        utils::log_sql(&sql);
        sql.execute(connection)
            .map(|num| {
                info!("Succeed to delete {} records", num);
                num
            })
            .map_err(ImmortalError::ignore)
    }
}

impl Handler<CategoryUpdate> for DBExecutor {
    type Result = Result<()>;
    fn handle(&mut self, message: CategoryUpdate, _: &mut Self::Context) -> Self::Result {
        let connection: &PgConnection = &self.0.get().unwrap();
        let sql = update(categories.find(message.0)).set(message.1);
        utils::log_sql(&sql);
        sql.execute(connection)
            .map(|_| {
                info!("Succeed to update record");
            })
            .map_err(ImmortalError::ignore)
    }
}
