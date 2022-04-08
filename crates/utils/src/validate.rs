use axum::{
    async_trait,
    extract::{Form, FromRequest, RequestParts},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

use crate::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatableForm<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatableJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatableForm<T>
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
            .map_err(|e| Error::ValidateError(collect_errors(e)))?;
        Ok(ValidatableForm(value))
    }
}

#[async_trait]
impl<T, B> FromRequest<B> for ValidatableJson<T>
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
            .map_err(|e| Error::ValidateError(collect_errors(e)))?;
        Ok(ValidatableJson(value))
    }
}

fn collect_errors(error: ValidationErrors) -> Vec<String> {
    error
        .field_errors()
        .into_iter()
        .map(|e| {
            let default_error = format!("{} is required", e.0);
            e.1[0]
                .message
                .as_ref()
                .unwrap_or(&std::borrow::Cow::Owned(default_error))
                .to_string()
        })
        .collect()
}
