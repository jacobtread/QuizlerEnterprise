use std::fmt::{Debug, Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{DbErr, TransactionError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;
use validator::ValidationError;

pub type HttpResult<T> = Result<T, TypedError>;

/// Trait implemented by HTTP error response types
pub trait HttpErrorResponse: std::error::Error + Send + Sync + 'static {
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

    /// Names for each of the error types, used for handling
    /// specific errors on the client
    fn name(&self) -> &'static str {
        "server"
    }
}

/// Type adapter that allows anyhow to meet the std::error::Error bounds
#[derive(Debug)]
pub struct AnyhowErrorAdapter(anyhow::Error);

impl Display for AnyhowErrorAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Internal server error")
    }
}

impl std::error::Error for AnyhowErrorAdapter {}

impl HttpErrorResponse for AnyhowErrorAdapter {}

impl HttpErrorResponse for DbErr {
    fn message(&self) -> String {
        "Internal server error".to_string()
    }
}

impl<E> From<E> for TypedError
where
    E: HttpErrorResponse,
{
    fn from(value: E) -> Self {
        TypedError::General(Box::new(value))
    }
}
impl<E> From<TransactionError<E>> for TypedError
where
    E: HttpErrorResponse + std::error::Error,
{
    fn from(value: TransactionError<E>) -> Self {
        match value {
            TransactionError::Connection(err) => err.into(),
            TransactionError::Transaction(err) => TypedError::General(Box::new(err)),
        }
    }
}

impl From<anyhow::Error> for TypedError {
    fn from(value: anyhow::Error) -> Self {
        Self::General(Box::new(AnyhowErrorAdapter(value)))
    }
}

#[derive(Debug, Error)]
pub enum TypedError {
    /// Dynamic HTTP error type
    #[error("{0}")]
    General(Box<dyn HttpErrorResponse>),
    /// Validation error
    #[error(transparent)]
    Validation(Box<ValidationError>),
}

impl TypedError {
    fn status_code(&self) -> StatusCode {
        match self {
            TypedError::General(msg) => msg.status_code(),
            TypedError::Validation(_) => StatusCode::BAD_REQUEST,
        }
    }
}

/// Error context in a format that can be serialized as JSON
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TypedErrorJson {
    /// General generic error messages
    General {
        /// The error name
        name: &'static str,
        /// The error message
        message: String,
    },
    /// Validation error messages with fields
    Validation(Box<ValidationError>),
}

impl IntoResponse for TypedError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let json = match self {
            TypedError::General(err) => {
                err.log();
                TypedErrorJson::General {
                    name: err.name(),
                    message: err.message(),
                }
            }
            TypedError::Validation(err) => TypedErrorJson::Validation(err),
        };
        (status_code, Json(json)).into_response()
    }
}
