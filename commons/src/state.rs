use actix_redis::RedisActor;
use actix_web::actix::Addr;

use crate::DBExecutor;

pub struct AppState {
    pub db: Addr<DBExecutor>,
    pub redis: Addr<RedisActor>,
}
