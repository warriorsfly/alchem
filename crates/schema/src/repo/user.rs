use crate::source::{LocalUser, NewLocalUser, NewUser, User};
use alchem_utils::{
    db::DieselConnection,
    encryption::{hash_password, verify_password},
    Error,
};
use diesel::{dsl::*, prelude::*};
use diesel_async::{AsyncConnection, RunQueryDsl};

pub async fn signup(
    conn: &mut DieselConnection,
    username: String,
    pwd: String,
) -> Result<User, Error> {
    use crate::schema::local_users::dsl::*;
    use crate::schema::users::dsl::*;
    conn.transaction::<_, User, Error>(|c| {
        Box::pin(async move {
            let existed = select(exists(users.filter(name.eq(&username))))
                .get_result(c)
                .await?;
            if existed {
                return Err(Error::BadRequest("Username already exists".to_string()));
            }
            let new_user = NewUser {
                name: username.as_str(),
                bio: "",
                avatar: "",
            };

            let user = insert_into(users)
                .values(&new_user)
                .get_result::<User>(c)
                .await?;

            let psw_hashed = hash_password(pwd.as_str())
                .map_err(|e| Error::InternalServerError(e.to_string()))?;

            let local_user = NewLocalUser {
                user_id: &user.id,
                password_encrypted: &psw_hashed.0,
                salt: &psw_hashed.1,
            };
            insert_into(local_users)
                .values(&local_user)
                .execute(c)
                .await?;

            Ok(user)
        })
    })
    .await
}

pub async fn login(
    conn: &mut DieselConnection,
    username: String,
    psw: String,
) -> Result<User, Error> {
    use crate::schema::local_users::dsl::*;
    use crate::schema::users::dsl::*;
    conn.transaction::<_, User, Error>(|c| {
        Box::pin(async move {
            let user: User = users.filter(name.eq(username)).get_result(c).await?;

            let local_user: LocalUser = local_users
                .filter(user_id.eq(&user.id))
                .get_result(c)
                .await?;

            verify_password(
                psw.as_str(),
                &local_user.salt,
                local_user.password_encrypted.as_bytes(),
            )
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

            Ok(user)
        })
    })
    .await
}

// #[cfg(test)]
// mod test {

//     use alchem_utils::db::get_connection;

//     use super::*;

//     fn exist_user<'a>(conn: &'a mut DbConnection, username: &'a str) -> Result<bool, Error> {
//         use crate::schema::users::dsl::*;
//         let user = users.filter(name.eq(username)).first::<User>(conn);
//         Ok(user.is_ok())
//     }

//     fn delete_user<'a>(conn: &'a mut DbConnection, username: &'a str) -> Result<(), Error> {
//         use crate::schema::users::dsl::*;
//         delete(users.filter(name.eq(username))).execute(conn)?;
//         Ok(())
//     }

//     #[test]
//     async fn test_signup() {
//         let  conn = &mut get_connection("postgres://allen:walker@127.0.0.1/double_zero").await.unwrap();
//         if exist_user(conn, "test").unwrap() {
//             let _ = delete_user(conn, "test").unwrap();
//         }
//         let user = signup(conn, "test", "test").unwrap();
//         assert_eq!(user.name, "test");
//     }

//     #[test]
//     async fn test_login() {
//         let conn = &mut get_connection("postgres://allen:walker@127.0.0.1/double_zero").await.unwrap();
//         let user = login(conn, "test", "test").unwrap();
//         assert_eq!(user.name, "test");
//     }
// }
