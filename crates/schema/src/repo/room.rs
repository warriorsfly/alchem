use crate::source::{NewRoom, NewRoomUser, Room, RoomUser};
use alw_utils::{db::DieselConnection, Error};
use diesel::{dsl::*, prelude::*};
use diesel_async::{AsyncConnection, RunQueryDsl};
use futures::FutureExt;

/// TODO: how to create the link?
pub async fn create_room(
    conn: &mut DieselConnection,
    usr_id: i32,
    room_name: String,
    link: String,
) -> Result<Room, Error> {
    use crate::schema::room_users::dsl::*;
    use crate::schema::rooms::dsl::*;
    conn.transaction::<Room, Error, _>(|c| {
        async move {
            let new_room = NewRoom {
                name: &room_name.as_str(),
                invite_link: &link.as_str(),
                owner: &usr_id,
            };
            let room = insert_into(rooms)
                .values(&new_room)
                .get_result::<Room>(c)
                .await?;

            let new_room_user = NewRoomUser {
                room_id: &room.id,
                user_id: &usr_id,
                is_admin: &true,
            };
            insert_into(room_users)
                .values(&new_room_user)
                .execute(c)
                .await?;

            Ok(room)
        }
        .boxed()
    })
    .await
}

pub async fn delete_room(
    conn: &mut DieselConnection,
    usr_id: i32,
    room_id: i32,
) -> Result<(), Error> {
    use crate::schema::rooms::dsl::*;
    conn.transaction::<(), Error, _>(|c| {
        async move {
            delete(rooms.filter(id.eq(room_id)).filter(owner.eq(usr_id)))
                .execute(c)
                .await?;

            Ok(())
        }
        .boxed()
    })
    .await
}

// pub async fn change_room_owner(
//     conn: &mut DieselConnection,
//     rid: i32,
//     ownr: i32,
//     usr_id: i32,
// ) -> Result<Room, Error> {
//     use crate::schema::room_users::dsl::*;
//     use crate::schema::rooms::dsl::*;
//     conn.transaction::<_, Room, Error>(|c| {
//         Box::pin(async move {
//             let room = update(rooms.find(id.eq(rid)))
//                 .set(owner.eq(usr_id))
//                 .get_result::<Room>(c)
//                 .await?;
//             // tokio::select!(

//             //     _ = delete(room_users.filter(room_id.eq(rid)))
//             //         .execute(c)
//             //         .fuse() => {},

//             //     _ = insert_into(room_users)
//             //         .values(&NewRoomUser {
//             //             room_id: &rid,
//             //             user_id: &ownr,
//             //             is_admin: &true,
//             //         })
//             //         .execute(c)
//             //         .fuse() => {},
//             // );

//             Ok(room)
//         })
//     })
//     .await
// }

pub async fn insert_room_user(
    conn: &mut DieselConnection,
    rm_id: i32,
    usr_id: i32,
) -> Result<RoomUser, Error> {
    use crate::schema::room_users::dsl::*;
    conn.transaction::<RoomUser, Error, _>(|c| {
        async move {
            let new_room_user = NewRoomUser {
                room_id: &rm_id,
                user_id: &usr_id,
                is_admin: &false,
            };
            let room_user = insert_into(room_users)
                .values(&new_room_user)
                .get_result::<RoomUser>(c)
                .await?;

            Ok(room_user)
        }
        .boxed()
    })
    .await
}

pub async fn delete_room_user(
    conn: &mut DieselConnection,
    rm_id: i32,
    usr_id: i32,
) -> Result<(), Error> {
    use crate::schema::room_users::dsl::*;

    delete(
        room_users
            .filter(user_id.eq(usr_id))
            .filter(room_id.eq(rm_id)),
    )
    .execute(conn)
    .await?;

    Ok(())
}

pub async fn get_user_rooms(conn: &mut DieselConnection, usr_id: i32) -> Result<Vec<Room>, Error> {
    use crate::schema::room_users::dsl::*;
    use crate::schema::rooms::dsl::*;
    let rms = rooms
        .inner_join(room_users)
        .filter(user_id.eq(usr_id))
        .load::<(Room, RoomUser)>(conn)
        .await?;

    Ok(rms.into_iter().map(|(r, _)| r).collect())
}
