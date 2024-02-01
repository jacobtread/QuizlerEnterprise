use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{DbErr, TransactionError};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

pub type HttpResult<T> = Result<T, HttpErrorResponse>;

/// Trait implemented by HTTP error response types
pub trait HttpError: std::error::Error + Send + Sync + 'static {
    /// Handles logging the error before its consumed
    fn log(&self) {
        error!(name: "err_http", error = %self);
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn message(&self) -> String {
        self.to_string()
    }

    /// Converts the error into the response
    fn into_response(self: Box<Self>) -> Response {
        (
            self.status_code(),
            Json(JsonErrorResponse {
                name: self.name(),
                message: self.message(),
                data: (),
            }),
        )
            .into_response()
    }

    /// Names for each of the error types, used for handling
    /// specific errors on the client
    fn name(&self) -> &'static str {
        "server"
    }
}

/// Wrapper around [JsonRejection] for changing the error response
/// format to match the standard format
#[derive(Debug, Error)]
#[error(transparent)]
pub struct JsonErrorAdapter(#[from] JsonRejection);

impl HttpError for JsonErrorAdapter {
    fn name(&self) -> &'static str {
        "json_parse"
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl From<JsonRejection> for HttpErrorResponse {
    fn from(value: JsonRejection) -> Self {
        Self(Box::new(JsonErrorAdapter(value)))
    }
}

/// Type adapter that allows anyhow to meet the std::error::Error bounds
#[derive(Debug, Error)]
#[error("Internal server error")]
pub struct AnyhowErrorAdapter(anyhow::Error);

impl HttpError for AnyhowErrorAdapter {}

impl From<anyhow::Error> for HttpErrorResponse {
    fn from(value: anyhow::Error) -> Self {
        Self(Box::new(AnyhowErrorAdapter(value)))
    }
}

impl HttpError for DbErr {
    fn message(&self) -> String {
        "Internal server error".to_string()
    }
}

impl<E> From<E> for HttpErrorResponse
where
    E: HttpError,
{
    fn from(value: E) -> Self {
        Self(Box::new(value))
    }
}
impl<E> From<TransactionError<E>> for HttpErrorResponse
where
    E: HttpError + std::error::Error,
{
    fn from(value: TransactionError<E>) -> Self {
        match value {
            TransactionError::Connection(err) => err.into(),
            TransactionError::Transaction(err) => Self(Box::new(err)),
        }
    }
}

/// Adapter for custom [HttpErrorResponse]'s from [ValidationErrors] containing
/// the additional validation message data
#[derive(Debug, Error)]
#[error(transparent)]
pub struct ValidationErrorAdapter(#[from] ValidationErrors);

impl HttpError for ValidationErrorAdapter {
    fn name(&self) -> &'static str {
        "validation"
    }

    fn message(&self) -> String {
        "Validation error ocurred".to_string()
    }

    fn into_response(self: Box<Self>) -> Response {
        (
            self.status_code(),
            Json(JsonErrorResponse {
                name: self.name(),
                message: self.message(),
                data: self.0,
            }),
        )
            .into_response()
    }
}

impl From<ValidationErrors> for HttpErrorResponse {
    fn from(value: ValidationErrors) -> Self {
        Self(Box::new(ValidationErrorAdapter(value)))
    }
}

/// Error response type wrapping some dynamic [HttpError]
/// type for creating responses
#[derive(Debug)]
pub struct HttpErrorResponse(Box<dyn HttpError>);

/// JSON structure for an error response with some generic data
/// value that can be provided
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonErrorResponse<D> {
    /// The error name
    pub name: &'static str,

    /// The error message
    pub message: String,

    /// Additional response data
    pub data: D,
}

impl IntoResponse for HttpErrorResponse {
    fn into_response(self) -> Response {
        HttpError::into_response(self.0)
    }
}
