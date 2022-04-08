use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};

pub mod apub;
pub mod claims;
pub mod config;
pub mod encryption;
pub mod pool;
pub mod validate;

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    Unauthorized(String),
    ValidateError(Vec<String>),
}

/// Convert DBErrors to ServiceErrors
impl From<DBError> for Error {
    fn from(error: DBError) -> Error {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::BadRequest(message);
                }
                Error::InternalServerError("Unknown database error".into())
            }
            _ => Error::InternalServerError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ServiceErrors
impl From<PoolError> for Error {
    fn from(error: PoolError) -> Error {
        Error::InternalServerError(error.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),

            Error::NotFound(message) => (StatusCode::NOT_FOUND, message),

            Error::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),

            Error::ValidateError(message) => (StatusCode::BAD_REQUEST, message.join("\n")),
        };

        (status, error_message).into_response()
    }
}

// pub mod utils;
/// local user id
pub type LocalUserId = usize;
/// websocket connection id
pub type ConnectionId = usize;
/// room id
pub type RoomId = usize;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct IpAddr(pub String);

pub struct WsServer {
    redis_cluster: redis::cluster::ClusterClient,
}

impl WsServer {
    pub fn new(redis_urls: Vec<&str>) -> Self {
        Self {
            redis_cluster: redis::cluster::ClusterClient::open(redis_urls).unwrap(),
        }
    }
}
