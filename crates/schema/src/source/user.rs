use serde::{Deserialize, Serialize};

use crate::schema::users;
#[derive(Clone, Queryable, Serialize, Deserialize,Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub avatar: String,
    pub bio: String,
    pub local: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    // pub display_name: &'a str,
    pub bio: &'a str,
    pub avatar: &'a str,
    // pub local: &'a bool,
}
