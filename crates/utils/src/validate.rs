use axum::{
    async_trait,
    extract::{Form, FromRequest, RequestParts},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req)
            .await
            .map_err(|e| Error::BadRequest(e.to_string()))?;
        value
            .validate()
            .map_err(|e| Error::ValidateError(e.to_string()))?;
        Ok(ValidatedForm(value))
    }
}

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req)
            .await
            .map_err(|e| Error::BadRequest(e.to_string()))?;
        value
            .validate()
            .map_err(|e| Error::ValidateError(e.to_string()))?;
        Ok(ValidatedJson(value))
    }
}
