use jwt_simple::prelude::*;

pub mod apub;
pub mod claims;
pub mod config;
pub mod pool;
// pub mod utils;
/// local user id
pub type LocalUserId = usize;
/// websocket connection id
pub type ConnectionId = usize;
/// room id
pub type RoomId = usize;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct IpAddr(pub String);


pub struct AppState {
    key_pair:RS384KeyPair,
    redis_cluster: redis::cluster::ClusterClient,
}

impl AppState {
    pub fn new(redis_urls: Vec<&str>) -> Self {
        Self {
            key_pair: RS384KeyPair::from_pem("keys/private.pem").expect("failed to load private key"),
            redis_cluster: redis::cluster::ClusterClient::open(redis_urls).unwrap(),
        }
    }
}
