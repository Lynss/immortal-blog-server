#![feature(async_await, futures_api, await_macro)]

extern crate actix_web;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate futures;
extern crate listenfd;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate serde_derive;

use actix_web::server;
use dotenv::dotenv;
use listenfd::ListenFd;
use server::{HttpServer, IntoHttpHandler};

use common::LOG_CONFIG;

mod common;
mod handler;
mod model;
mod router;

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
            self.bind("127.0.0.1:3000").unwrap()
        }
    }
}

fn main() {
    dotenv().ok();
    log4rs::init_file(LOG_CONFIG, Default::default()).unwrap();
    info!("Server started on http://localhost:3000");
    server::new(router::init_with_state).hot_listen().run();
}
