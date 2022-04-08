use crate::source::{LocalUser, NewLocalUser, NewUser, User};
use alchem_utils::{
    encryption::{hash_password, verify_password},
    pool::{DbPool, DatabaseConnection},
    Error,
};
use diesel::{dsl::*, prelude::*};

pub fn signup<'a>(DatabaseConnection(conn): DatabaseConnection, username: &'a str, pwd: &'a str) -> Result<User, Error> {
    use crate::schema::local_users::dsl::*;
    use crate::schema::users::dsl::*;
    let mut conn = conn;

    let existed = select(exists(users.filter(name.eq(username)))).get_result(&mut conn);
    if let Ok(true) = existed {
        return Err(Error::BadRequest("username existed".to_string()));
    }
    let usr = NewUser {
        name: username,
        bio: "",
        avatar: "",
    };

    let psw_hashed = hash_password(pwd).map_err(|e| Error::InternalServerError(e.to_string()))?;

    conn.transaction::<User, _, _>(|c| {
        let user = diesel::insert_into(users)
            .values(usr)
            .get_result::<User>(c)?;

        let local_user = NewLocalUser {
            user_id: &user.id,
            password_encrypted: &psw_hashed.0,
            salt: &psw_hashed.1,
        };
        let _ = diesel::insert_into(local_users)
            .values(local_user)
            .execute(c)?;

        Ok(user)
    })
}

pub fn login<'a>(pool: &'a DbPool, username: &'a str, psw: &'a str) -> Result<User, Error> {
    use crate::schema::local_users::dsl::*;
    use crate::schema::users::dsl::*;
    let mut conn = pool.get()?;

    conn.transaction::<User, _, _>(|c| {
        let user: User = users.filter(name.eq(username)).get_result(c)?;

        let local_user: LocalUser = local_users.filter(user_id.eq(&user.id)).get_result(c)?;

        verify_password(
            &local_user.password_encrypted,
            psw,
            local_user.salt.as_bytes(),
        )
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

        Ok(user)
    })
}

#[cfg(test)]
mod test {
    use alchem_utils::pool::init_pool;

    use super::*;

    fn exist_user(db: &DbPool, username: &str) -> Result<bool, Error> {
        use crate::schema::users::dsl::*;
        let conn = &mut db.get()?;
        let user = users.filter(name.eq(username)).first::<User>(conn);
        Ok(user.is_ok())
    }

    fn delete_user(db: &DbPool, username: &str) -> Result<(), Error> {
        use crate::schema::users::dsl::*;
        let conn = &mut db.get()?;
        delete(users.filter(name.eq(username))).execute(conn)?;
        Ok(())
    }

    // #[test]
    // fn test_signup() {
    //     let pool = init_pool("postgres://allen:walker@127.0.0.1/double_zero");
    //     if exist_user(&pool, "test").unwrap() {
    //         let _ = delete_user(&pool, "test").unwrap();
    //     }
    //     let user = signup(&pool, "test", "test").unwrap();
    //     assert_eq!(user.name, "test");
    // }

    // #[test]
    // fn test_login() {
    //     let pool = init_pool("postgres://allen:walker@127.0.0.1/double_zero");
    //     let user = login(&pool, "test", "test").unwrap();
    //     assert_eq!(user.name, "test");
    // }
}
