use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};


use alchem_utils::{claims::PrivateClaims, config::CONFIG, Error};
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
use redis::Commands;
use tokio::sync::{broadcast, mpsc};

pub struct Group {
    pub name: String,
    pub broad: broadcast::Sender<String>,
}

pub struct SocketServer {
    pub redis_cluster: redis::cluster::ClusterClient,
    pub rooms: RwLock<HashMap<i32, Group>>,
    /// users in connected to current server
    pub users: RwLock<HashMap<i32, mpsc::Sender<String>>>,
}

impl SocketServer {
    pub fn new() -> Self {
        Self {
            redis_cluster: redis::cluster::ClusterClient::open(
                CONFIG.redis_cluster_nodes.split(",").collect(),
            )
            .expect("Unable to connect to redis cluster"),
            rooms: RwLock::new(HashMap::with_capacity(1)),
            users: RwLock::new(HashMap::with_capacity(1)),
        }
    }
}

impl SocketServer {
    /// get user's online info, whether local or not, if not local,
    /// go to redis hset see
    fn is_user_socket_in_local(&self, user: i32) -> bool {
        self.users.try_read().unwrap().contains_key(&user)
    }
    fn hset_user_room(&self, user: i32, room: i32) -> Result<(), Error> {
        let cluster_con = self
            .redis_cluster
            .get_connection()
            .map_err(|e| Error::InternalServerError(e.to_string()))?;
        let _result = cluster_con.hset_nx(format!("user:{}", user), "rooms", room)?;
        Ok(())
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
    Extension(app): Extension<Arc<SocketServer>>,
    claim: PrivateClaims,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    let user = claim.id;
    ws.on_upgrade(move |stream| handle_socket(stream, app, user))
}

async fn handle_socket(stream: WebSocket, app: Arc<SocketServer>, user: i32) {
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
