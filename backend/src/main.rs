#[macro_use]
extern crate log;
#[macro_use]
extern crate redis_async;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use common::{configs::BACKEND_LOG_CONFIG, RedisActor};
use listenfd::ListenFd;
use middlewares::Cors;
use share::DBExecutor;
use state::AppState;
use std::env;

mod handlers;
mod middlewares;
mod state;
mod utils;

fn main() {
    utils::ready_env();
    log4rs::init_file(BACKEND_LOG_CONFIG, Default::default()).unwrap();
    let backend_server_address = env::var("BACKEND_SERVER_ADDRESS").unwrap();

    let app = move || {
        let db_addr = DBExecutor::init();
        let redis_address = env::var("REDIS_ADDRESS").unwrap();
        let backend_client_address = env::var("BACKEND_CLIENT_ADDRESS").unwrap();
        let redis_addr = RedisActor::start(redis_address.clone());
        let origins = vec![backend_client_address];
        App::new()
            .data(AppState {
                db: db_addr.clone(),
                redis: redis_addr.clone(),
            })
            .wrap(Logger::default())
            .wrap(Cors::new(origins))
            .service(web::resource("/upload").route(web::post().to_async(handlers::upload_file)))
            //static files
            .service(Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/ws").route(web::get().to(handlers::ws_message_handler)))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/roles").service(
                            web::resource("/options")
                                .route(web::get().to_async(handlers::get_role_options)),
                        ),
                    )
                    .service(
                        web::scope("/users")
                            .service(
                                web::resource("").route(web::get().to_async(handlers::get_users)),
                            )
                            .service(
                                web::resource("/forbidden")
                                    .route(web::put().to_async(handlers::forbid_users)),
                            )
                            .service(
                                web::resource("/activated_email")
                                    .route(web::post().to_async(handlers::send_activated_email)),
                            )
                            .service(
                                web::scope("/settings")
                                    .service(
                                        web::resource("").route(
                                            web::get().to_async(handlers::get_user_settings),
                                        ),
                                    )
                                    .service(web::resource("/{id}").route(
                                        web::put().to_async(handlers::update_user_settings),
                                    )),
                            )
                            .service(
                                web::resource("/activation")
                                    .route(web::put().to_async(handlers::active_user)),
                            )
                            .service(
                                web::resource("/options")
                                    .route(web::get().to_async(handlers::get_author_options)),
                            )
                            .service(
                                web::resource("/is_repeated")
                                    .route(web::get().to_async(handlers::check_repeated_user)),
                            ),
                    )
                    .service(
                        web::scope("/tags")
                            .service(
                                web::resource("")
                                    .route(web::get().to_async(handlers::get_tags))
                                    .route(web::post().to_async(handlers::create_tag))
                                    .route(web::delete().to_async(handlers::delete_tag)),
                            )
                            .service(
                                web::resource("/options")
                                    .route(web::get().to_async(handlers::get_author_options)),
                            )
                            .service(
                                web::resource("/{id}")
                                    .route(web::put().to_async(handlers::update_tag)),
                            ),
                    )
                    .service(
                        web::scope("/categories")
                            .service(
                                web::resource("")
                                    .route(web::get().to_async(handlers::get_categories))
                                    .route(web::post().to_async(handlers::create_category))
                                    .route(web::delete().to_async(handlers::delete_category)),
                            )
                            .service(
                                web::resource("/options")
                                    .route(web::get().to_async(handlers::get_author_options)),
                            )
                            .service(
                                web::resource("/{id}")
                                    .route(web::put().to_async(handlers::update_category)),
                            ),
                    )
                    .service(web::resource("/login").route(web::post().to_async(handlers::login)))
                    .service(
                        web::resource("/register").route(web::post().to_async(handlers::register)),
                    ),
            )
    };

    let mut server = HttpServer::new(app);

    info!("Server started on {}", backend_server_address);
    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("localhost:8083").unwrap()
    };
    let _ = server.run();
}
