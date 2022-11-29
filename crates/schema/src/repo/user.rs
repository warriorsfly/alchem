use crate::source::{LocalUser, NewLocalUser, NewRoomUser, NewUser, RoomUser, User};
use alw_utils::{
    db::DieselConnection,
    encryption::{hash_password, verify_password},
    Error,
};
use diesel::{dsl::*, prelude::*};
use diesel_async::{AsyncConnection, RunQueryDsl};
use futures::FutureExt;

pub async fn signup(
    conn: &mut DieselConnection,
    username: String,
    pwd: String,
) -> Result<User, Error> {
    use crate::schema::local_users::dsl::*;
    use crate::schema::users::dsl::*;
    conn.transaction::<User, Error, _>(|c| {
        async move {
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
        }
        .boxed()
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
    conn.transaction::<User, Error, _>(|c| {
        async move {
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
        }
        .boxed()
    })
    .await
}

pub async fn join_room(conn: &mut DieselConnection, usr: i32, rm: i32) -> Result<RoomUser, Error> {
    use crate::schema::room_users::dsl::*;
    conn.transaction::<RoomUser, Error, _>(|c| {
        async move {
            let existed = select(exists(
                room_users.filter(user_id.eq(usr)).filter(room_id.eq(rm)),
            ))
            .get_result(c)
            .await?;
            if existed {
                return Err(Error::BadRequest("user is in the room already".to_string()));
            }
            let new_room_user = NewRoomUser {
                room_id: &rm,
                user_id: &usr,
                is_admin: &false,
            };

            let rusr = insert_into(room_users)
                .values(&new_room_user)
                .get_result::<RoomUser>(c)
                .await?;

            Ok(rusr)
        }
        .boxed()
    })
    .await
}

// #[cfg(test)]
// mod test {

//     use alw_utils::db::get_connection;

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
