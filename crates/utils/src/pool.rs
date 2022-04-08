use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    Extension,
};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use tracing::error;

use crate::Error;

// #[cfg(feature = "postgres")]
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
// #[cfg(feature = "postgres")]
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub struct DatabaseConnection(pub DbConnection);

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<DbPool>::from_request(req)
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;

        let conn = tokio::task::spawn_blocking({
            move || {
                pool.get().map_err(|e| {
                    error!("Failed to get connection from pool: {}", e);
                    Error::InternalServerError(e.to_string())
                })
            }
        })
        .await
        .map_err(|e| {
            error!("Failed to join task to runtime: {}", e);

            Error::InternalServerError("Failed to join task to runtime".into())
        })??;

        Ok(Self(conn))
    }
}

pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("database_url error")
}
