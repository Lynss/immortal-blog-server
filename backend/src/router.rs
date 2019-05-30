use actix_redis::RedisSession;
use actix_web::{
    http::Method,
    middleware::{session::SessionStorage, Logger},
    web, App,
};

use crate::{handlers, middlewares::{Auth,Cors}, AppState};
use common::RedisActor;
use share::DBExecutor;
use std::env;

pub fn init_with_state<T, B>() -> App<T, B> {
    let db_addr = DBExecutor::init();
    let redis_address = env::var("REDIS_ADDRESS").unwrap();
    let backend_client_address = env::var("BACKEND_CLIENT_ADDRESS").unwrap();
    let redis_addr = RedisActor::start(redis_address);
    let origins = vec![backend_client_address.as_str()];
    App.new()
        .data(AppState {
            db: db_addr.clone(),
            redis: redis_addr.clone(),
        })
        .wrap(Logger::default())
        .wrap(Cors::new(origins))
        .wrap(Auth)
        // redis session middleware
        .wrap(RedisSession::new(redis_address.as_str(), &[0; 32]))
        .service(
            web::scope("/api")
                .service(
                    web::resource("/privileges")
                        .route(web::get().to_async(handlers::get_privileges)),
                )
                .service(
                    web::resource("/tags")
                        .route(web::post().to_async(handlers::create_tag))
                        .route(web::get().to_async(handlers::get_tags)),
                )
                .service(web::resource("/login").route(web::post().to_async(handlers::login)))
                .service(
                    web::resource("/register").route(web::post().to_async(handlers::register)),
                ),
        )
}
