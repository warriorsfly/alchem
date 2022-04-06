use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    Extension,
};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use tracing::error;

// #[cfg(feature = "postgres")]
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
// #[cfg(feature = "postgres")]
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
pub struct DatabaseConnection(Connection);

#[async_trait]
impl<B> FromRequest<B> for DatabaseConnection
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(pool) = Extension::<DbPool>::from_request(req)
            .await
            .map_err(internal_error)?;

        let conn = tokio::task::spawn_blocking({
            move || {
                pool.get().map_err(|e| {
                    error!("Failed to get connection from pool: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to get connection from pool: {}", e),
                    )
                })
            }
        })
        .await
        .map_err(|e| {
            error!("Failed to join task to runtime: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to join task to runtime".into(),
            )
        })??;

        Ok(Self(conn))
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
