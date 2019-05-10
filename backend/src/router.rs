use actix_web::{http::Method, middleware::Logger, App};
use commons::{middlewares::Cors, AppState, DBExecutor, RedisActor};

use crate::{handlers,middlewares::Auth};

pub fn init_with_state() -> App<AppState> {
    let db_addr = DBExecutor::init();
    let redis_addr = RedisActor::start("127.0.0.1:6379");
    let origins = vec!["http://localhost:3000"];
    App::with_state(AppState {
        db: db_addr.clone(),
        redis: redis_addr.clone(),
    })
    .middleware(Logger::default())
    .middleware(Cors::new(origins))
    .middleware(Auth)
    .scope("/api", |api| {
        api.resource("/privileges", |route| {
            route
                .method(Method::GET)
                .with_async(handlers::get_privileges)
        })
        .resource("/login", |route| {
            route.method(Method::POST).with_async(handlers::login)
        })
        .resource("/user", |route| {
            route.method(Method::GET).with_async(handlers::get_users)
        })
    })
}
