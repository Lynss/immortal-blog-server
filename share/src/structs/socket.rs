use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use common::{
    configs::{CLIENT_TIMEOUT, EVENTSOURCE, HEARTBEAT_INTERVAL, USER_PREFIX_KEY},
    utils,
};
use futures::future;
use redis_async::{client, resp::FromResp};
use std::{env, time::Instant};

pub struct WebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    user_id: i32,
}

impl Actor for WebSocket {
    type Context = WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Message, ProtocolError> for WebSocket {
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        // process websocket messages
        match msg {
            Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Message::Pong(_) => {
                self.hb = Instant::now();
            }
            Message::Text(text) => {
                ctx.text(text);
            }
            Message::Binary(bin) => ctx.binary(bin),
            Message::Close(_) => {
                ctx.stop();
            }
            Message::Nop => (),
        }
    }
}

impl WebSocket {
    pub fn new(user_id: i32) -> Self {
        Self {
            hb: Instant::now(),
            user_id,
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        let topic = format!(
            "{}:{}",
            utils::create_prefix_key(USER_PREFIX_KEY, self.user_id),
            EVENTSOURCE
        );
        let redis_address = env::var("REDIS_ADDRESS").unwrap().parse().unwrap();
        let msgs = client::pubsub_connect(&redis_address)
            .and_then(move |connection| connection.subscribe(&topic));
        msgs.map_err(|_| ()).and_then(|msgs| {
            msgs.for_each(|message| {
                println!("{}", String::from_resp(message).unwrap());
                ctx.text("123");
                future::ok(())
            })
        });
        ctx.run_interval(HEARTBEAT_INTERVAL, move |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                warn!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }
            ctx.ping("");
        });
    }
}
