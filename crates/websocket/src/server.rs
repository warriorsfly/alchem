use std::{collections::HashSet, sync::Arc};

use crate::{MessageOpration, RoomOperation};
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
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use redis::{
    cluster::cluster_pipe,
    streams::{StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply},
    Commands, FromRedisValue, ToRedisArgs,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, PartialEq)]
#[serde(tag = "type", content = "id")]
pub enum AlcReceiver {
    /// user who is the receiver
    User(i32),
    /// group of the users in it are the receivers
    Room(i32),
}

#[derive(Serialize, Deserialize)]
pub struct AlcRawMessage {
    /// the sender of the message
    pub sende: i32,
    pub recv: AlcReceiver,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct AlcMessage {
    /// the sender of the message
    pub sende: i32,
    pub recv: AlcReceiver,
    pub message: String,
    pub time: i64,
}


impl FromRedisValue for AlcReceiver {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref val) => match serde_json::from_slice(val) {
                Err(_) => Err(((redis::ErrorKind::TypeError, "Can't serialize value")).into()),
                Ok(v) => Ok(v),
            },
            _ => Err(((
                redis::ErrorKind::ResponseError,
                "Response type not Dashboard compatible.",
            ))
                .into()),
        }
    }
}

impl FromRedisValue for AlcMessage {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref val) => match serde_json::from_slice(val) {
                Err(_) => Err(((redis::ErrorKind::TypeError, "Can't serialize value")).into()),
                Ok(v) => Ok(v),
            },
            _ => Err(((
                redis::ErrorKind::ResponseError,
                "Response type not Dashboard compatible.",
            ))
                .into()),
        }
    }
}

impl ToRedisArgs for AlcReceiver {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match &self {
            Self::Room(id) => {
                "type".write_redis_args(out);
                "room".write_redis_args(out);
                "id".write_redis_args(out);
                id.write_redis_args(out);
            }

            Self::User(id) => {
                "type".write_redis_args(out);
                "user".write_redis_args(out);
                "id".write_redis_args(out);
                id.write_redis_args(out);
            }
        }
    }
}

impl ToRedisArgs for AlcMessage {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        "sende".write_redis_args(out);
        let _ = &self.sende.write_redis_args(out);

        "recv".write_redis_args(out);
        let _ = &self.recv.write_redis_args(out);

        "message".write_redis_args(out);
        let _ = &self.message.write_redis_args(out);

        "time".write_redis_args(out);
        let _ = &self.time.write_redis_args(out);
    }
}

pub struct SocketServer {
    pub redis_cluster: redis::cluster::ClusterClient,
    opts: StreamReadOptions,
}

pub async fn init_socket_server() -> SocketServer {
    SocketServer {
        redis_cluster: redis::cluster::ClusterClient::open(
            CONFIG.redis_cluster_url.split(",").collect(),
        )
        .expect("Unable to connect to redis cluster"),
        opts: StreamReadOptions::default().block(5000).count(10),
    }
}

impl RoomOperation for SocketServer {
    fn create_room(&self, rid: i32, rname: &str, owner: i32) -> Result<(), Error> {
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

    fn join_room(&self, user: i32, room: i32) -> Result<(), Error> {
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

    fn leave_room(&self, user: i32, room: i32) -> Result<(), Error> {
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

    fn get_my_rooms(&self, user: i32) -> Result<HashSet<i32>, Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let rooms = connection.smembers(format!("rooms-of-user:{}", user))?;
        Ok(rooms)
    }
}

impl MessageOpration for SocketServer {
    fn send_message(&self, msg: AlcMessage) -> Result<(), Error> {
        let maxlen = StreamMaxlen::Approx(3000);
        match &msg.recv {
            AlcReceiver::Room(room) => {
                let connection = &mut self.redis_cluster.get_connection()?;
                let usrs: HashSet<i32> = connection.smembers(format!("users-in-room:{}", room))?;
                for usr in usrs {
                    let _: () =
                        connection.xadd_maxlen_map("alc-message-user", maxlen, usr, &msg)?;
                }
            }
            AlcReceiver::User(user_id) => {
                let connection = &mut self.redis_cluster.get_connection()?;
                let _: () =
                    connection.xadd_maxlen_map("alc-message-user", maxlen, user_id, &msg)?;
            }
        }
        Ok(())
    }

    fn receive_message(&self, user: i32) -> Result<Vec<AlcMessage>, Error> {
        let connection = &mut self.redis_cluster.get_connection()?;
        let last_message_id: String =
            connection.hget(format!("user:{}", user), "last-message-id").unwrap_or("0-0".to_string());
        let ssr: StreamReadReply =
            connection.xread_options(&["alc-message-user"], &[&last_message_id], &self.opts)?;
        let mut messages: Vec<AlcMessage> = Vec::new();
        for StreamKey { key: _, ids } in ssr.keys {
  
            if ids.len() == 0 {
                continue;
            }
            let items: Vec<AlcMessage> = ids.iter().map(|t| AlcMessage { sende: t.get("sende").unwrap(), recv: t.get("recv").unwrap(), message: t.get("message").unwrap(), time: t.get("time").unwrap() }).collect();
            messages.extend(items);

            let max_message_id = ids.last().unwrap().id.clone();

            let _ = connection.hset(format!("user:{}", user), "last-message-id", max_message_id)?;
      
        }

        Ok(messages)
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
    ws.on_upgrade(move |sock| handle_socket(sock, srv, user))
}

async fn handle_socket(sock: WebSocket, srv: Arc<SocketServer>, user: i32) {
    let (mut sink, mut stream) = sock.split();
    // handle messages from client
    while let Some(msg) = stream.next().await {
        if let Ok(Message::Text(txt)) = msg {
            let msg = serde_json::from_str(&txt);
            if let Ok(AlcRawMessage {
                sende,
                recv,
                message,
            }) = msg
            {
                let msg = AlcMessage {
                    sende,
                    recv,
                    message,
                    time: Utc::now().timestamp(),
                };

                let _ = srv.send_message(msg);
            }
        }
    }

    // handle messages from server
    loop {
        let messages = srv.receive_message(user);

        if let Ok(items) = messages {
            if items.len() > 0 {
                let res = serde_json::to_string(&items);
                if let Ok(res) = res {
                    if sink.send(Message::Text(res)).await.is_err() {
                        println!("client disconnected");
                        return;
                    }
                }
            }
        }
    }
}
