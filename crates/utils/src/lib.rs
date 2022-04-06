pub mod apub;
pub mod pool;
// pub mod rate_limit;
// pub mod settings;
// pub mod utils;
/// local user id
pub type LocalUserId = usize;
/// websocket connection id
pub type ConnectionId = usize;
/// room id
pub type RoomId = usize;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct IpAddr(pub String);
