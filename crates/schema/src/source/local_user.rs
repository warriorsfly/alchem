use crate::schema::local_users;

#[derive(Clone, Queryable)]
#[diesel(table_name =local_users)]
pub struct LocalUser {
    pub id: i32,
    pub user_id: i32,
    pub password_encrypted: String,
    pub salt: String,
    pub phone: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = local_users)]
pub struct NewLocalUser<'a> {
    pub user_id: &'a i32,
    pub password_encrypted: &'a str,
    pub salt: &'a str,
}
