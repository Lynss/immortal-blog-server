use crate::utils;
use actix_web::{web::Payload, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use common::{ImmortalError, Result};
use share::structs::WebSocket;

/// do websocket handshake and start `WebSocket` actor
pub fn ws_message_handler(req: HttpRequest, stream: Payload) -> Result<HttpResponse> {
    utils::get_user_id_from_header(&req).and_then(|id| {
        let res = ws::start(WebSocket::new(id), &req, stream).map_err(ImmortalError::ignore);
        println!("{:?}", res.as_ref().unwrap());
        res
    })
}
