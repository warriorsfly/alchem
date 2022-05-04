use serde::Serialize;

use crate::schema::room_users;

#[derive(Clone, Queryable,Serialize)]
#[diesel(table_name =room_users)]
pub struct RoomUser {
    pub id: i32,
    pub room_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

#[derive(Debug, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = room_users)]
pub struct NewRoomUser<'a> {
    pub room_id: &'a i32,
    pub user_id: &'a i32,
    pub is_admin: &'a bool,
}
