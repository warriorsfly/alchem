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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct IpAddr(pub String);

pub struct Daoism {
    pub redis_client: redis::Client,
    pub rooms: RwLock<HashMap<i32, HashSet<i32>>>,
    /// users in connected to current server
    pub users: RwLock<HashSet<i32>>,
}

impl Daoism {
    pub fn new() -> Self {
        Self {
            redis_client: redis::Client::open(CONFIG.redis_url.as_ref()).unwrap(),
            rooms: RwLock::new(HashMap::with_capacity(1)),
            users: RwLock::new(HashSet::with_capacity(1)),
        }
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(app): Extension<Arc<Daoism>>,
    claim: PrivateClaims,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    let mut users = app.users.try_write().unwrap();
    users.insert(claim.id);
    // let cn = app
    //     .redis_client
    //     .get_multiplexed_tokio_connection()
    //     .await
    //     .unwrap();

    // cn.hset("all".to_string(), "all".to_string(), "all".to_string());

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("client send str: {:?}", t);
                }
                Message::Binary(_) => {
                    println!("client send binary data");
                }
                Message::Ping(_) => {
                    println!("socket ping");
                }
                Message::Pong(_) => {
                    println!("socket pong");
                }
                Message::Close(_) => {
                    println!("client disconnected");
                    return;
                }
            }
        } else {
            println!("client disconnected");
            return;
        }
    }

    loop {
        if socket
            .send(Message::Text(String::from("Hi!")))
            .await
            .is_err()
        {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
