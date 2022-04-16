use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use alchem_utils::claims::PrivateClaims;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    headers,
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc};

pub struct Group {
    pub name: String,
    pub broad: broadcast::Sender<String>,
}

pub struct WebsocketServer {
    // pub redis_client: redis::c,
    pub rooms: RwLock<HashMap<i32, Group>>,
    /// users in connected to current server
    pub users: RwLock<HashMap<i32, mpsc::Sender<String>>>,
}

impl WebsocketServer {
    pub fn new() -> Self {
        Self {
            // redis_client: redis::Client::open(CONFIG.redis_url.as_ref()).unwrap(),
            rooms: RwLock::new(HashMap::with_capacity(1)),
            users: RwLock::new(HashMap::with_capacity(1)),
        }
    }
}

impl WebsocketServer {
    /// get user's online info, weather local or not, if not local,
    /// go to redis hset see
    fn is_user_local(&self, user: i32) -> bool {
        self.users.try_read().unwrap().contains_key(&user)
    }

    // async fn get_user_grpc_addr(&self,user: i32)->String{
    //     self.redis_client.get_multiplexed_tokio_connection().await.unwrap().hget(
    //         "user_info",
    //         user.to_string(),
    //     ).unwrap().unwrap()
    // }
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
    let (mut sink, _stream) = stream.split();
    let mut users = app.users.try_write().unwrap();
    let (tx, mut rx) = mpsc::channel(1);
    users.insert(user, tx);

    let _tsk = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sink.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // let rcon = app.redis_client.get_multiplexed_tokio_connection().await.unwrap();
}
