#![feature(async_await, await_macro)]
extern crate actix_redis;
extern crate actix_web;
extern crate chrono;
extern crate common;
extern crate diesel;
extern crate futures;
extern crate listenfd;
extern crate share;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate redis_async;
extern crate actix_session;

use actix_web::server;
use listenfd::ListenFd;
use server::{HttpServer, IntoHttpHandler};
use std::env;
use common::configs::BACKEND_LOG_CONFIG;
use state::*;

mod handlers;
mod middlewares;
mod router;
mod state;
mod utils;

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
        let backend_server_address = env::var("BACKEND_SERVER_ADDRESS").unwrap();
        if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
            self.listen(l)
        } else {
            self.bind(backend_server_address).unwrap()
        }
    }
}

fn main() {
    utils::ready_env();
    log4rs::init_file(BACKEND_LOG_CONFIG, Default::default()).unwrap();
    let backend_server_address = env::var("BACKEND_SERVER_ADDRESS").unwrap();
    info!("Server started on {}", backend_server_address);
    server::new(router::init_with_state).hot_listen().run();
}
