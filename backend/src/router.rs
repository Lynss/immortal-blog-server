use actix_web::{
    App,
    http::{header, Method},
    middleware::{cors::Cors, Logger},
};

use crate::{
    handlers,
    models::{AppState, DBExecutor},
};

pub fn init_with_state() -> App<AppState> {
    let db_addr = DBExecutor::init();
    App::with_state(AppState {
        db: db_addr.clone(),
    })
    .middleware(Logger::default())
    .configure(|app| {
        Cors::for_app(app)
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .max_age(3600)
            .resource("/api/user", |r| {
                r.method(Method::GET).with_async(handlers::get_users)
            })
            .register()
    })
}
