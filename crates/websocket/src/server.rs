use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
};

use alchem_utils::{claims::PrivateClaims, config::CONFIG, Error};
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    headers,
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
// use pulsar::{
//     message::proto::command_subscribe::SubType, message::Payload, Consumer, DeserializeMessage,
//     Pulsar, TokioExecutor, reader::Reader,
// };

use redis::{cluster::cluster_pipe, Commands};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub user: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<i32>,
    pub message: String,
}

pub struct SocketServer {
    pub redis_cluster: redis::cluster::ClusterClient,
    /// users in connected to current server
    pub users: RwLock<HashMap<i32, mpsc::Sender<String>>>,
}

pub async fn init_socket_server() -> SocketServer {
    SocketServer {
        redis_cluster: redis::cluster::ClusterClient::open(
            CONFIG.redis_cluster_url.split(",").collect(),
        )
        .expect("Unable to connect to redis cluster"),
        // plsar: Pulsar::builder(CONFIG.ems_url.as_str(), TokioExecutor)
        //     .build()
        //     .await
        //     .expect("Unable to connect to pulsar"),

        users: RwLock::new(HashMap::with_capacity(1)),
    }
}

impl SocketServer {
    /// get user's online info, whether local or not, if not local,
    /// go to redis hset see
    pub fn is_user_online_in_local(&self, user: i32) -> bool {
        self.users.try_read().unwrap().contains_key(&user)
    }
    pub fn create_room(&self, rid: i32, rname: &str, owner: i32) -> Result<(), Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let _: () = cluster_pipe()
            // create room entity in redis
            .hset(format!("room:{}", rid), "owner", owner)
            .ignore()
            .hset(format!("room:{}", rid), "name", rname)
            .ignore()
            // create users in room hashset
            .hset(
                format!("users-in-room:{}", rid),
                // user id
                owner,
                // user on which alchem server, inited at the time when it upgrade to websocket
                "",
            )
            .ignore()
            // add room id to user's rooms hashset
            .sadd(format!("rooms-of-user:{}", owner), rid)
            .ignore()
            .query(connection)?;

        Ok(())
    }

    pub fn join_room(&self, user: i32, room: i32) -> Result<(), Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let _: () = cluster_pipe()
            // create users in room hashset
            .hset(
                format!("users-in-room:{}", room),
                // user id
                user,
                // user on which alchem server, the value changed when user's websocket online
                "",
            )
            .ignore()
            // add room id to user's rooms hashset
            .sadd(format!("rooms-of-user:{}", user), room)
            .ignore()
            .query(connection)?;
        Ok(())
    }

    pub fn leave_room(&self, user: i32, room: i32) -> Result<(), Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let _: () = cluster_pipe()
            // create users in room hashset
            .hdel(
                format!("users-in-room:{}", room),
                // user id
                user,
            )
            .ignore()
            // add room id to user's rooms hashset
            .srem(format!("rooms-of-user:{}", user), room)
            .ignore()
            .query(connection)?;
        Ok(())
    }

    pub fn get_user_rooms(&self, user: i32) -> Result<HashSet<i32>, Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let rooms = connection.smembers(format!("rooms-of-user:{}", user))?;
        Ok(rooms)
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(srv): Extension<Arc<SocketServer>>,
    claim: PrivateClaims,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    let user = claim.id;
    ws.on_upgrade(move |stream| handle_socket(stream, srv, user))
}

async fn handle_socket(stream: WebSocket, srv: Arc<SocketServer>, user: i32){
    let (mut sink, _stream) = stream.split();
    // let mut users = srv.users.try_write().unwrap();
    // let (tx, mut rx) = mpsc::channel(1);
    // users.insert(user, tx);
    let rooms = srv.get_user_rooms(user).unwrap();

       let _tsk = tokio::spawn(async move {
    //   rooms.iter().for_each(|room|  {
    //     let mut consumer:Reader<ChatMessage, TokioExecutor> = srv
    //         .plsar
    //         .reader()
    //         .with_topic(format!("room-message-{}", room))
    //         .with_lookup_namespace("namespace")
    //         .with_consumer_name(format!("user-{}", user))
    //         .with_subscription_type(SubType::Shared)
    //         .with_subscription("room-message-subscription")
    //         .build()
    //         .await.unwrap();
    //   });
    });
    // rooms.iter().for_each(|room| async {
    //     let mut consumer:Consumer<ChatMessage, TokioExecutor> = srv
    //         .plsar
    //         .consumer()
    //         .with_topic(format!("room-message-{}", room))
    //         .with_lookup_namespace("namespace")
    //         .with_consumer_name(format!("user-{}", user))
    //         .with_subscription_type(SubType::Shared)
    //         .with_subscription("room-message-subscription")
    //         .build()
    //         .await.unwrap();

    //          while let Some(msg) = consumer.try_next().await? {
    //     consumer.ack(&msg).await?;
    //     let data = match msg.deserialize() {
    //         Ok(data) => data,
    //         Err(e) => {
    //             log::error!("could not deserialize message: {:?}", e);
    //             break;
    //         }
    //     };

    //     if data.data.as_str() != "data" {
    //         log::error!("Unexpected payload: {}", &data.data);
    //         break;
    //     }
    //     counter += 1;
    //     log::info!("got {} messages", counter);
    // }
    // });

    // let _tsk = tokio::spawn(async move {
    //     while let Some(msg) = rx.recv().await {
    //         // In any websocket error, break loop.
    //         if sink.send(Message::Text(msg)).await.is_err() {
    //             break;
    //         }
    //     }
    // });

    // let rcon = app.redis_redis_clusterent.get_multiplexed_tokio_connection().await.unwrap();
}
