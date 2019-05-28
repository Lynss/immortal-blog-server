use actix_redis::RedisSessionBackend;
use actix_web::{
    http::Method,
    middleware::{session::SessionStorage, Logger},
    App,
};

use crate::{handlers, middlewares::Auth, AppState};
use common::{middlewares::Cors, RedisActor};
use share::DBExecutor;
use std::env;

pub fn init_with_state() -> App<AppState> {
    let db_addr = DBExecutor::init();
    let redis_address = env::var("REDIS_ADDRESS").unwrap();
    let backend_client_address = env::var("BACKEND_CLIENT_ADDRESS").unwrap();
    let redis_addr = RedisActor::start(redis_address);
    let origins = vec![backend_client_address.as_str()];
    App::with_state(AppState {
        db: db_addr.clone(),
        redis: redis_addr.clone(),
    })
    .middleware(Logger::default())
    .middleware(Cors::new(origins))
    .middleware(Auth)
    // redis session middleware
    .middleware(SessionStorage::new(RedisSessionBackend::new(
        redis_address.as_str(),
        &[0; 32],
    )))
    .scope("/api", |api| {
        api.resource("/privileges", |route| {
            route
                .method(Method::GET)
                .with_async(handlers::get_privileges);
        })
        .resource("/tags", |route| {
            route.method(Method::POST).with_async(handlers::create_tag);
            route.method(Method::GET).with_async(handlers::get_tags);
        })
        .resource("/login", |route| {
            route.method(Method::POST).with_async(handlers::login);
        })
        .resource("/register", |route| {
            route.method(Method::POST).with_async(handlers::register);
        })
    })
}
