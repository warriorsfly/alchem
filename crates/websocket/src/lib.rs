use std::sync::Arc;

use alchem_utils::config::CONFIG;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    headers,
    response::IntoResponse,
    Extension,
};

/// local user id
pub type LocalUserId = usize;
/// websocket connection id
pub type ConnectionId = usize;
/// room id
pub type RoomId = usize;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct IpAddr(pub String);

pub struct Application {
    pub redis_client: redis::Client,
}

impl Application {
    pub fn new() -> Self {
        Self {
            redis_client: redis::Client::open(CONFIG.redis_url.as_ref()).unwrap(),
        }
    }
}

// pub type AioConnection = redis::aio::MultiplexedConnection;
// pub struct RedisConnection(pub AioConnection);

// #[async_trait]
// impl<B> FromRequest<B> for RedisConnection
// where
//     B: Send,
// {
//     type Rejection = Error;

//     async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//         let Extension(app) = Extension::<Application>::from_request(req)
//             .await
//             .map_err(|e| Error::InternalServerError(e.to_string()))?;

//         let conn = app
//             .redis_client
//             .get_multiplexed_tokio_connection()
//             .await
//             .map_err(|e| Error::InternalServerError(e.to_string()))?;

//         Ok(Self(conn))
//     }
// }

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(app): Extension<Arc<Application>>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    let cn = app
        .redis_client
        .get_multiplexed_tokio_connection()
        .await
        .unwrap();

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
