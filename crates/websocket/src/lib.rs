mod handlers;
mod server;
use std::collections::HashSet;

use alchem_utils::Error;


pub use self::server::*;

trait RoomOperation {
    fn create_room(&self, rid: i32, rname: &str, owner: i32) -> Result<(), Error>;
    fn join_room(&self, user: i32, room: i32) -> Result<(), Error>;
    fn leave_room(&self, user: i32, room: i32) -> Result<(), Error>;
    fn get_my_rooms(&self, user: i32) -> Result<HashSet<i32>, Error>;
}

trait MessageOpration {
    fn send_message(&self, msg: AlcMessage) -> Result<(), Error>;
    fn receive_message(&self, user: i32) -> Result<Vec<AlcMessage>, Error>;
}
