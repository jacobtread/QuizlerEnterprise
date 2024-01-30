use std::sync::Arc;

use crate::{
    database::entities::user::User,
    http::models::error::{HttpErrorResponse, TypedError},
    services::auth::{AuthService, TokenError},
};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    typed_header::TypedHeaderRejection,
    TypedHeader,
};
use sea_orm::DatabaseConnection;
use thiserror::Error;

/// Middleware for authorizing users through tokens, contains
/// the authorized user
pub struct Auth(pub User);

/// Middleware for gating authorization without actually looking
/// up the user in question
pub struct AuthGate;

#[derive(Debug, Error)]
pub enum AuthError {
    /// Header was missing or invalid
    #[error(transparent)]
    Header(TypedHeaderRejection),
    /// Token was expired or invalid
    #[error("Invalid token")]
    Token(#[from] TokenError),
    /// Token user no longer exists
    #[error("Invalid token")]
    UnknownUser,
}

impl HttpErrorResponse for AuthError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            AuthError::Header(_) | AuthError::Token(_) | AuthError::UnknownUser => {
                StatusCode::BAD_REQUEST
            }
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = TypedError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth: Arc<AuthService> = parts
            .extensions
            .get::<Arc<AuthService>>()
            .expect("Missing auth service")
            .clone();
        let db: DatabaseConnection = parts
            .extensions
            .get::<DatabaseConnection>()
            .expect("Missing auth service")
            .clone();
        let TypedHeader(authorization) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(AuthError::Header)?;
        let token = authorization.token();
        let claims = auth.verify_user_token(token).map_err(AuthError::Token)?;
        let user = User::find_by_id(&db, claims.user_id)
            .await?
            .ok_or(AuthError::UnknownUser)?;

        Ok(Self(user))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthGate
where
    S: Send + Sync,
{
    type Rejection = TypedError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth: Arc<AuthService> = parts
            .extensions
            .get::<Arc<AuthService>>()
            .expect("Missing auth service")
            .clone();
        let TypedHeader(authorization) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(AuthError::Header)?;
        let token = authorization.token();
        let _claims = auth.verify_user_token(token).map_err(AuthError::Token)?;

        Ok(Self)
    }
}
