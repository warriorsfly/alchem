
use crate::schema::local_users;
use serde::{Deserialize, Serialize};

#[derive(Clone, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name =local_users)]
pub struct LocalUser {
  pub id: i32,
  pub user_id: i32,
  pub password_encrypted: String,
  pub salt: String,
  pub phone: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = local_users)]
pub struct NewLocalUser<'a> {
    pub user_id: &'a i32,
    pub password_encrypted: &'a str,
    pub salt: &'a str,
}
