use std::collections::HashSet;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use futures::stream::SplitSink;
use std::net::SocketAddr;
// use tower_http::{
//     services::ServeDir,
//     trace::{DefaultMakeSpan, TraceLayer},
// };

pub type ConnectionId = usize;
pub type UserId = usize;
pub type RoomId = usize;
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct IpAddr(pub String);

pub struct WsSession {
    // pub socket: WebSocket,
    pub ip: IpAddr,
    pub communiity_rooms: Option<HashSet<RoomId>>,
}

pub struct AppState {
    redis: redis::cluster::ClusterClient,
}

impl AppState {
    pub fn new(redis_urls: Vec<&str>) -> Self {
        Self {
            redis: redis::cluster::ClusterClient::open(redis_urls).unwrap(),
        }
    }
}

