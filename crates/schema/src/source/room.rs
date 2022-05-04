use serde::{Deserialize, Serialize};
use crate::schema::rooms;
#[derive(Clone, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = rooms)]
#[diesel(belongs_to(User))]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub ico: String,
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
