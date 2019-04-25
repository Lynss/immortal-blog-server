use actix_web::actix::Addr;

use super::DBExecutor;

pub struct AppState {
    pub db: Addr<DBExecutor>,
}
