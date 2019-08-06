use crate::utils as share_utils;
use actix::fut::wrap_future;
use actix::prelude::*;
use actix::{Actor, Addr, Message as ActixMessage, StreamHandler};
use actix_redis::{Command, RespValue};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use chrono::NaiveDateTime;
use common::configs::EVENT_QUEUE;
use common::{
    configs::{CLIENT_TIMEOUT, EVENTSOURCE, HEARTBEAT_INTERVAL},
    utils, ImmortalError, RedisActor,
};
use redis_async::{
    client,
    error::{self, Error},
    resp::FromResp,
};
use std::{env, time::Instant};

fn fetch_messages(
    id: i32,
    redis: &Addr<RedisActor>,
) -> impl Future<Item = Messengers, Error = ImmortalError> {
    //try to pull messages from redis and push them to web client
    let message_queue = utils::create_prefix_key(EVENT_QUEUE, id);
    redis
        .send(Command(resp_array!["LRANGE", &message_queue, "0", "-1"]))
        .map_err(ImmortalError::ignore)
        .and_then(|res| match res {
            Ok(msgs @ RespValue::Array(_)) => Ok(Messengers {
                inner: Vec::<Messenger>::from_resp(msgs).unwrap(),
            }),
            _ => Err(ImmortalError::ignore(
                "Failed to get message from redis event queue",
            )),
        })
}

#[derive(Deserialize, Serialize)]
pub struct Messenger {
    pub message_type: String,
    pub title: String,
    pub content: String,
    pub href: Option<String>,
    pub img: Option<String>,
    #[serde(with = "utils::date_format")]
    pub created_at: NaiveDateTime,
}

#[derive(ActixMessage)]
pub struct Messengers {
    inner: Vec<Messenger>,
}

impl FromResp for Messenger {
    fn from_resp_int(resp: RespValue) -> Result<Self, Error> {
        match resp {
            RespValue::BulkString(ref bytes) => {
                let messenger_string: String = String::from_utf8_lossy(bytes).into_owned();
                let messenger = serde_json::from_str(messenger_string.as_str()).unwrap();
                Ok(messenger)
            }
            _ => Err(error::resp("Cannot convert into a messenger", resp)),
        }
    }
}

impl StreamHandler<Messengers, ImmortalError> for WebSocket {
    fn handle(&mut self, item: Messengers, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&item.inner).unwrap());
    }
}

pub struct WebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    user_id: i32,
    redis_addr: Addr<RedisActor>,
}

impl Actor for WebSocket {
    type Context = WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        let redis_address = env::var("REDIS_ADDRESS").unwrap().parse().unwrap();
        let user_id = self.user_id;
        let redis = &self.redis_addr;
        let redis = redis.clone();
        ctx.add_stream(
            client::pubsub_connect(&redis_address)
                .and_then(move |connection| connection.subscribe(EVENTSOURCE))
                .map_err(ImmortalError::ignore)
                .and_then(move |msgs| {
                    Ok(msgs
                        .map_err(ImmortalError::ignore)
                        .map(|message| String::from_resp(message).unwrap())
                        .filter(move |message| message == &user_id.to_string())
                        .and_then(move |_| fetch_messages(user_id, &redis)))
                })
                .flatten_stream(),
        );
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
                let user_id = self.user_id;
                let redis = &self.redis_addr;
                let redis = redis.clone();
                match text.as_str() {
                    "@init" => {
                        //Send initial messages to client
                        ctx.spawn(wrap_future(
                            share_utils::notify_fetch_message(&redis, &vec![user_id])
                                .map_err(|_| ()),
                        ));
                    }
                    "@clear-all" => {
                        //Send clear messages to client
                        ctx.spawn(wrap_future(
                            share_utils::clear_message(&redis, user_id)
                                .and_then(move |_| {
                                    share_utils::notify_fetch_message(&redis, &vec![user_id])
                                })
                                .map_err(|_| ()),
                        ));
                    }
                    _ => ctx.text(text),
                };
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
    pub fn new(user_id: i32, redis_addr: Addr<RedisActor>) -> Self {
        Self {
            hb: Instant::now(),
            user_id,
            redis_addr,
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
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
