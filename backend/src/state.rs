use actix_redis::RedisActor;
use actix::Addr;

use share::DBExecutor;

#[derive(Clone)]
pub struct AppState {
    pub db: Addr<DBExecutor>,
    pub redis: Addr<RedisActor>,
}
