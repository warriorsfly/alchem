use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use alchem_schema::source::Room;
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
use redis::{cluster::cluster_pipe, Commands};
use tokio::sync::{broadcast, mpsc};

pub struct SocketServer {
    pub redis_cluster: redis::cluster::ClusterClient,
    pub rooms: RwLock<HashMap<i32, broadcast::Sender<String>>>,
    /// users in connected to current server
    pub users: RwLock<HashMap<i32, mpsc::Sender<String>>>,
}

const KEY_ROOM: &str = "room";

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
    fn is_user_online_in_local(&self, user: i32) -> bool {
        self.users.try_read().unwrap().contains_key(&user)
    }

    fn create_room(&self, room: Room) -> Result<(), Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let _: () = cluster_pipe()
            // create room entity in redis
            .hset(format!("room:{}", &room.id), "owner", &room.owner).ignore()
            .hset(format!("room:{}", &room.id), "name", &room.name).ignore()
            // create users in room hashset
            .hset(
                format!("users-in-room:{}", &room.id),
                // user id
                &room.owner,
                // user on which alchem server, inited at the time when it upgrade to websocket
                "",
            )
            .ignore()
            // add room id to user's rooms hashset
            .sadd(format!("rooms-of-user:{}",&room.owner),  &room.id)
            .ignore()
            .query(connection)?;

        Ok(())
    }

    fn join_room(&self, user: i32, room: i32) -> Result<(), Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let _: () = cluster_pipe()
            // create users in room hashset
            .hset(
                format!("users-in-room:{}", room),
                // user id
                user,
                // user on which alchem server, inited at the time when it upgrade to websocket
                "",
            )
            .ignore()
            // add room id to user's rooms hashset
            .sadd(format!("rooms-of-user:{}",user),  room)
            .ignore()
            .query(connection)?;
        Ok(())
    }

    fn leave_room(&self, user: i32, room: i32) -> Result<(), Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let _: () = cluster_pipe()
            // create users in room hashset
            .hdel(
                format!("users-in-room:{}", room),
                // user id
                user
            )
            .ignore()
            // add room id to user's rooms hashset
            .srem(format!("rooms-of-user:{}",user),  room)
            .ignore()
            .query(connection)?;
        Ok(())
    }

    fn get_user_rooms(&self, user: i32) -> Result<HashSet<i32>, Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let rooms = connection.smembers(format!("rooms-of-user:{}", user))?;
        Ok(rooms)
    }

    // fn hdel_user_room(&self, user: i32, room: i32) -> Result<(), Error> {
    //     let connection =&mut self
    //         .redis_cluster
    //         .get_connection()
    //         .map_err(|e| Error::InternalServerError(e.to_string()))?;
    //     let _result = connection.hdel(format!("user:{}", user), "rooms", room)?;
    //     Ok(())
    // }

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
