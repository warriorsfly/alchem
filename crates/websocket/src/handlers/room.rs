use std::sync::Arc;

use alchem_schema::{repo, source::Room};
use alchem_utils::{claims::PrivateClaims, db::DatabaseConnection, validate::ValidatedForm, Error};
use axum::{Extension, Json};
use serde::Deserialize;
use validator::Validate;

use crate::server::SocketServer;

#[derive(Debug, Deserialize, Validate)]
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
    srv.create_room(&room)?;
    Ok(Json(room))
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
