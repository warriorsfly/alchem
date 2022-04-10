use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{config::KEY_PAIR, Error};

#[derive(Serialize, Deserialize)]
pub struct PrivateClaims {
    pub id: i32,
}

#[async_trait]
impl<B> FromRequest<B> for PrivateClaims
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|e| Error::Unauthorized(e.to_string()))?;

        let claim = KEY_PAIR
            .public_key()
            .verify_token::<PrivateClaims>(bearer.token(), None)
            .map_err(|e| Error::Unauthorized(e.to_string()))?;

        Ok(claim.custom)
    }
}
