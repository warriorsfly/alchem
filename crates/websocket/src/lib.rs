use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use alchem_utils::{claims::PrivateClaims, config::CONFIG};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    headers,
    response::IntoResponse,
    Extension,
};
use futures::{stream::SplitSink, StreamExt};
use tokio::sync::broadcast;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct IpAddr(pub String);

pub struct Room{
    pub name: String,
    pub broad:broadcast::Sender<String>,
}

pub struct WebsocketServer {
    pub redis_client: redis::Client,
    pub rooms: RwLock<HashMap<i32, HashSet<Room>>>,
    /// users in connected to current server
    pub users: RwLock<HashMap<i32, SplitSink<WebSocket, Message>>>,
}

impl WebsocketServer {
    pub fn new() -> Self {
        Self {
            redis_client: redis::Client::open(CONFIG.redis_url.as_ref()).unwrap(),
            rooms: RwLock::new(HashMap::with_capacity(1)),
            users: RwLock::new(HashMap::with_capacity(1)),
        }
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(app): Extension<Arc<WebsocketServer>>,
    claim: PrivateClaims,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    let user = claim.id;
    ws.on_upgrade(move |stream| handle_socket(stream, app, user))
}

async fn handle_socket(stream: WebSocket, app: Arc<WebsocketServer>, user: i32) {
    let (sink, mut stream) = stream.split();
    let mut users = app.users.try_write().unwrap();
    users.insert(user, sink);
}
