#![feature(async_await,custom_attribute, await_macro)]
extern crate actix_redis;
extern crate actix_web;
extern crate chrono;
extern crate commons;
#[macro_use]
extern crate diesel;
extern crate futures;
extern crate listenfd;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;
extern crate actix_redis;

use actix_web::server;
use listenfd::ListenFd;
use server::{HttpServer, IntoHttpHandler};

use commons::{configs::BACKEND_LOG_CONFIG,utils};

mod handlers;
mod router;
mod middlewares;
mod db_executors;

pub trait HotListener {
    fn hot_listen(self) -> Self;
}

impl<H, F> HotListener for HttpServer<H, F>
where
    H: IntoHttpHandler + 'static,
    F: Fn() -> H + Send + Clone + 'static,
{
    fn hot_listen(self) -> Self {
        let mut listenfd = ListenFd::from_env();
        if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
            self.listen(l)
        } else {
            self.bind("127.0.0.1:8083").unwrap()
        }
    }
}

fn main() {
    utils::ready_env();
    log4rs::init_file(BACKEND_LOG_CONFIG, Default::default()).unwrap();
    info!("Server started on http://localhost:8083");
    server::new(router::init_with_state).hot_listen().run();
}
