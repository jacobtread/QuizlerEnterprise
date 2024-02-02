use crate::http::models::error::HttpErrorResponse;
use async_trait::async_trait;
use axum::extract::{FromRequest, Request};
use serde::de::DeserializeOwned;

/// Wrapper around [Json] providing the correct error response structures
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(HttpErrorResponse))]
pub struct ExtractJson<T>(pub T);

/// Wrapper around [ExtractJson] that validates the deserialized result
/// responding with any validation errors
pub struct ValidJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + garde::Validate<Context = ()>,
{
    type Rejection = HttpErrorResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let ExtractJson(value) = ExtractJson::<T>::from_request(req, state).await?;
        value.validate(&())?;
        Ok(Self(value))
    }
}
