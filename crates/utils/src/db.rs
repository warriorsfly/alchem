use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use diesel_async::AsyncConnection;

use crate::{config::CONFIG, Error};

pub type DieselConnection = diesel_async::AsyncPgConnection;
pub struct DatabaseConnection(pub DieselConnection);

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(_req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(Self(
            DieselConnection::establish(CONFIG.database_url.as_str())
                .await
                .expect("Error connecting to database"),
        ))
    }
}
