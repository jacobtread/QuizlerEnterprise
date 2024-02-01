use axum::http::StatusCode;
use openid::{IdToken, StandardClaims};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use thiserror::Error;
use validator::Validate;

use crate::{
    services::auth::{AuthProvider, UserTokenData},
    utils::types::{EmailAddress, TransformValidate, Username},
};

use super::error::HttpError;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to create token, try logging in again")]
    FailedTokenIssue,
    /// Account already exists
    #[error("That email address is already in use")]
    EmailExists,
    /// Account email already exists
    #[error("That username is already in use")]
    UsernameExists,
}

impl HttpError for AuthError {
    fn name(&self) -> &'static str {
        match self {
            AuthError::FailedTokenIssue => "auth:token_create_failed",
            AuthError::EmailExists => "auth:email_exists",
            AuthError::UsernameExists => "auth:username_exists",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::FailedTokenIssue => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::EmailExists | AuthError::UsernameExists => StatusCode::CONFLICT,
        }
    }
}

#[derive(Debug, Error)]
pub enum OIDError {
    /// Tried to login using a provider that isn't linked to an
    /// existing account
    #[error(
        "An account already exists with the same email, please use the existing \
        account password. Once logged in you can link your account in settings"
    )]
    NotLinked,
    /// Failed to access the auth provider
    #[error("That authentication provider is currently unavailable, try again later.")]
    ProviderUnavailable,
    /// OpenID token is invalid
    #[error("Authentication token is invalid, try again.")]
    InvalidToken,
    /// Failed to authenticate with the provider
    #[error("Failed to authenticate with OpenID provider")]
    Authentication,
    /// Token claim was missing an email, OAuth is likely mis-configured
    #[error("Failed to determine account email address.")]
    ClaimMissingEmail,
}

impl HttpError for OIDError {
    fn name(&self) -> &'static str {
        match self {
            OIDError::NotLinked => "oid:not_linked",
            OIDError::ProviderUnavailable => "oid:provider_unavailable",
            OIDError::InvalidToken => "oid:invalid_token",
            OIDError::Authentication => "oid:auth_failed",
            OIDError::ClaimMissingEmail => "oid:claim_missing_email",
        }
    }

    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            OIDError::NotLinked => StatusCode::CONFLICT,
            OIDError::ProviderUnavailable => StatusCode::INTERNAL_SERVER_ERROR,
            OIDError::InvalidToken => StatusCode::BAD_REQUEST,
            OIDError::Authentication => StatusCode::BAD_REQUEST,
            OIDError::ClaimMissingEmail => StatusCode::BAD_REQUEST,
        }
    }
}

/// Request to check an OpenID token with the provider
#[derive(Deserialize)]
pub struct OIDConfirmRequest {
    /// The provider the token is from
    pub provider: AuthProvider,
    /// The token itself
    pub token: IdToken<StandardClaims>,
}

/// Request to refresh a token
#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    /// The token itself
    pub refresh_token: String,
}

/// Request for creating an account from an OpenID token
#[derive(Deserialize, Validate)]
pub struct OIDCreateRequest {
    /// The provider the token is from
    pub provider: AuthProvider,
    /// The token itself
    pub token: IdToken<StandardClaims>,
    /// The username for the user
    pub username: Username,
    /// The password to use for the user
    #[validate(length(
        min = 4,
        max = 100,
        message = "Password must be within 4 to 100 characters long"
    ))]
    pub password: String,
}

impl TransformValidate for OIDCreateRequest {
    fn transform_validate(&mut self) -> Result<(), validator::ValidationErrors> {
        self.username.transform_validate()?;
        Ok(())
    }
}

/// Request to authenticate an OpenID code
#[derive(Deserialize)]
pub struct OIDAuthenticateRequest {
    /// The provider the code is from
    pub provider: AuthProvider,
    /// The auth code
    pub code: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum OIDAuthenticateResponse {
    /// Account doesn't exist, prepare account creation
    CreateAccount {
        /// The auth token to authenticate with OpenID
        token: Box<IdToken<StandardClaims>>,
        /// The default username based on the name present in the claim
        default_username: Option<Username>,
    },
    /// Account exists and is linked to this method, logged in
    ExistingLinked(TokenResponse),
}

/// Response containing an authorization token
#[derive(Serialize)]
pub struct TokenResponse {
    /// The auth token to log the user in with
    #[serde(flatten)]
    pub user_token_data: UserTokenData,
}

/// Response containing the available OpenID auth providers
#[serde_as]
#[derive(Serialize)]
pub struct OIDProvidersResponse {
    /// Collection of available providers
    #[serde_as(as = "serde_with::Map<_, _>")]
    pub providers: Vec<(AuthProvider, OIDProvider)>,
}

/// Details about an OpenID auth provider
#[derive(Serialize)]
pub struct OIDProvider {
    /// The URL for authenticating with the provider
    pub auth_url: Url,
}

/// Request to register an account with basic details
#[derive(Deserialize, Validate)]
pub struct BasicRegisterRequest {
    /// The username for the user
    pub username: Username,
    /// The email for the user
    pub email: EmailAddress,
    /// The password to use for the user
    #[validate(length(
        min = 4,
        max = 100,
        message = "Password must be within 4 to 100 characters long"
    ))]
    pub password: String,
}

impl TransformValidate for BasicRegisterRequest {
    fn transform_validate(&mut self) -> Result<(), validator::ValidationErrors> {
        self.username.transform_validate();
        self.email.transform_validate();
        Ok(())
    }
}
