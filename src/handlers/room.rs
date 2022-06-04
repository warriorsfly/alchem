use std::sync::Arc;

use alchem_schema::{repo, source::{Room, RoomUser}};
use alchem_utils::{claims::PrivateClaims, db::DatabaseConnection, validate::ValidatedForm, Error};
use alchem_websocket::{SocketServer, RoomOpt};
use axum::{Extension, Json, extract::Path};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug,Serialize,Deserialize, Validate)]
pub struct RoomForm {
    #[validate(length(max = 255))]
    pub name: String,
}

pub(crate) async fn create_room_handler(
    Extension(srv): Extension<Arc<SocketServer>>,
    DatabaseConnection(mut conn): DatabaseConnection,
    private_claims: PrivateClaims,
    ValidatedForm(entity): ValidatedForm<RoomForm>,
) -> Result<Json<Room>, Error> {
    let room = repo::create_room(&mut conn, private_claims.id, entity.name, "".to_string()).await?;
    srv.create_room(room.id,&room.name.as_str(),room.owner)?;
    Ok(Json(room))
}

pub(crate) async fn create_room_handler_2(
    ValidatedForm(entity): ValidatedForm<RoomForm>,
) -> Result<Json<RoomForm>, Error> {
    Ok(Json(entity))
}

// pub(crate) async fn change_room_owner_handler(
//     Extension(srv): Extension<Arc<SocketServer>>,
//     DatabaseConnection(mut conn): DatabaseConnection,
//     private_claims: PrivateClaims,
//     ValidatedForm(entity): ValidatedForm<RoomForm>,
// ) -> Result<Json<Room>, Error> {
//     let room = repo::create_room(&mut conn, private_claims.id, entity.name, "".to_string()).await?;
//     srv.create_room(&room)?;
//     Ok(Json(room))
// }

pub(crate) async fn join_room_handler(
    Extension(srv): Extension<Arc<SocketServer>>,
    DatabaseConnection(mut conn): DatabaseConnection,
    private_claims: PrivateClaims,
    Path(room_id): Path<i32>,
) -> Result<Json<RoomUser>, Error> {
    let room = repo::join_room(&mut conn, private_claims.id, room_id).await?;
    srv.join_room(private_claims.id, room_id)?;
    Ok(Json(room))
}
