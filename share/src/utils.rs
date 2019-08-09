use crate::structs::Messenger;
use actix::Addr;
use actix_redis::{Command, RespValue};
use common::{
    configs::{EVENTSOURCE, EVENT_QUEUE},
    utils as common_utils, ImmortalError, RedisActor,
};
use futures::{future::join_all, Future};

pub fn notify_fetch_message(
    redis: &Addr<RedisActor>,
    ids: &Vec<i32>,
) -> impl Future<Item = (), Error = ImmortalError> {
    join_all(
        ids.iter()
            .map(|id| {
                let id = id.to_string();
                redis
                    .send(Command(resp_array!["PUBLISH", EVENTSOURCE, id]))
                    .map_err(ImmortalError::ignore)
                    .and_then(|result| match result {
                        Ok(RespValue::Integer(_)) => Ok(()),
                        _ => Err(ImmortalError::ignore(
                            "Error during publish message to topic by redis",
                        )),
                    })
            })
            .collect::<Vec<_>>(),
    )
    .map(|_| ())
}

pub fn produce_message(
    redis: &Addr<RedisActor>,
    messages: Vec<(i32, Vec<Messenger>)>,
) -> impl Future<Item = (), Error = ImmortalError> {
    join_all(
        messages
            .iter()
            .map(|(id, messengers)| {
                let message_queue = common_utils::create_prefix_key(EVENT_QUEUE, id.to_owned());
                let mut messengers = messengers
                    .iter()
                    .map(|messenger| serde_json::to_string(messenger).unwrap())
                    .collect::<Vec<String>>();
                redis
                    .send(Command(
                        resp_array!["LPUSH", &message_queue].append(&mut messengers),
                    ))
                    .map_err(ImmortalError::ignore)
                    .and_then(move |result| match result {
                        Ok(RespValue::Integer(_)) => Ok(()),
                        _ => {
                            let message_queue = message_queue.to_owned();
                            Err(ImmortalError::ignore(format!(
                                "Error during produce message with key {}",
                                message_queue
                            )))
                        }
                    })
            })
            .collect::<Vec<_>>(),
    )
    .map(|_| ())
}

pub fn clear_message(
    redis: &Addr<RedisActor>,
    id: i32,
) -> impl Future<Item = (), Error = ImmortalError> {
    let message_queue = common_utils::create_prefix_key(EVENT_QUEUE, id.to_owned());
    redis
        .send(Command(resp_array!["DEL", &message_queue]))
        .map_err(ImmortalError::ignore)
        .and_then(move |result| match result {
            Ok(RespValue::Integer(_)) => Ok(()),
            _ => {
                let message_queue = message_queue.to_owned();
                Err(ImmortalError::ignore(format!(
                    "Error during delete message with key {}",
                    message_queue
                )))
            }
        })
}
