use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use num_cpus;
use std::env;

pub struct DBExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

impl DBExecutor {
    pub fn init() -> Addr<Self> {
        let db_url = env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        //create database pool
        let conn = Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to create pool.");
        info!("cpu nums:{}", num_cpus::get());
        SyncArbiter::start(4, move || DBExecutor(conn.clone()))
    }
}
