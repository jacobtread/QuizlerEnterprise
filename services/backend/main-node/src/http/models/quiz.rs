use axum::http::StatusCode;
use serde::Deserialize;
use thiserror::Error;

use super::error::HttpError;

#[derive(Debug, Error)]
pub enum QuizError {
    /// No matching Quiz found
    #[error("Quiz not found")]
    NotFound,
    /// No permission to access
    #[error("Missing permission")]
    MissingPermission,
}

impl HttpError for QuizError {
    fn name(&self) -> &'static str {
        match self {
            QuizError::NotFound => "quiz:not_found",
            QuizError::MissingPermission => "quiz:missing_permission",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            QuizError::NotFound => StatusCode::NOT_FOUND,
            QuizError::MissingPermission => StatusCode::FORBIDDEN,
        }
    }
}

/// Request to create a quiz
#[derive(Deserialize, garde::Validate)]
pub struct CreateQuizRequest {
    /// The title of the quiz
    #[garde(length(min = 4, max = 100))]
    pub title: String,
}
