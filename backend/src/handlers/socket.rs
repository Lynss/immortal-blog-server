use crate::utils;
use crate::AppState;
use actix_web::{
    web::{Data, Payload, Query},
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use common::{Claims, ImmortalError, Result};
use share::structs::{TokenBox, WebSocket};

/// do websocket handshake and start `WebSocket` actor
pub fn ws_message_handler(
    req: HttpRequest,
    state: Data<AppState>,
    token_box: Query<TokenBox>,
    stream: Payload,
) -> Result<HttpResponse> {
    let token = &token_box.token;
    utils::jwt_decode(token.clone(), None).and_then(|claims: Claims| {
        let redis_addr = state.redis.clone();
        ws::start(WebSocket::new(claims.id, redis_addr), &req, stream)
            .map_err(ImmortalError::ignore)
    })
}
