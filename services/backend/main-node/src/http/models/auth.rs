use openid::{IdToken, StandardClaims};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

use crate::services::auth::AuthProvider;

use super::error::HttpErrorResponse;

pub type AuthToken = String;

#[derive(Debug, Error)]
pub enum OIDError {
    /// Tried to login using a provider that isn't linked to an
    /// existing account
    #[error(
        "An account already exists with the same email, please use the existing\
        account password. Once logged in you can link your account in settings"
    )]
    NotLinked,
    /// Failed to access the auth provider
    #[error("That authentication provider is currently unavailable, try again later.")]
    ProviderUnavailable,
    /// OpenID token is invalid
    #[error("Authentication token is invalid, try again.")]
    Token,
    /// Token claim was missing an email, OAuth is likely mis-configured
    #[error("Failed to determine account email address.")]
    ClaimMissingEmail,
    /// Account already exists
    #[error("An account with a matching email already exists")]
    AlreadyExists,
}

impl HttpErrorResponse for OIDError {}

/// Request to check an OpenID token with the provider
#[derive(Deserialize)]
pub struct OIDConfirmRequest {
    /// The provider the token is from
    pub provider: AuthProvider,
    /// The token itself
    pub token: IdToken<StandardClaims>,
}

/// Response from handling an OpenID token with a specific provider
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum OIDConfirmResponse {
    /// The OpenID token was successfully confirmed
    Success {
        /// The default username based on the name present in the claim
        default_username: Option<String>,
    },

    /// An account already exists and is setup with the provided auth
    /// provider so a auth token is provided instead
    Existing {
        /// The auth token to log the user in with
        token: AuthToken,
    },
}

/// Request for creating an account from an OpenID token
#[derive(Deserialize, Validate)]
pub struct OIDCreateRequest {
    /// The provider the token is from
    pub provider: AuthProvider,
    /// The token itself
    pub token: IdToken<StandardClaims>,
    /// The username for the user
    #[validate(length(
        min = 4,
        max = 100,
        message = "Username must be within 4 to 100 characters long"
    ))]
    pub username: String,
    /// The password to use for the user
    #[validate(length(
        min = 4,
        max = 100,
        message = "Password must be within 4 to 100 characters long"
    ))]
    pub password: String,
}

/// Response for after an account is created with an OpenID token
#[derive(Serialize)]
pub struct OIDCreateResponse {
    /// The auth token to log the user in with
    pub token: AuthToken,
}
