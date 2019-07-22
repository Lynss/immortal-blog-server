use actix::Addr;
use actix_redis::RedisActor;

use share::DBExecutor;

#[derive(Clone)]
pub struct AppState {
    pub db: Addr<DBExecutor>,
    pub redis: Addr<RedisActor>,
}
