use axum::http::StatusCode;
use serde::Deserialize;
use thiserror::Error;

use super::error::HttpError;

#[derive(Debug, Error)]
pub enum QuizError {
    /// No matching Quiz found
    #[error("Quiz not found")]
    QuizNotFound,
}

impl HttpError for QuizError {
    fn name(&self) -> &'static str {
        match self {
            QuizError::QuizNotFound => "auth:email_not_found",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            QuizError::QuizNotFound => StatusCode::NOT_FOUND,
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
