use serde::{Deserialize, Serialize};

use crate::schema::room_users;
use crate::schema::rooms;
#[derive(Clone, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub invite_link: String,
    pub owner: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub name: &'a str,
    pub invite_link: &'a str,
    pub owner: &'a i32,
}

#[derive(Clone, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = room_users)]
pub struct RoomUser {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = room_users)]
pub struct NewRoomUser<'a> {
    pub room_id: &'a i32,
    pub user_id: &'a i32,
    pub is_admin: &'a bool,
}
