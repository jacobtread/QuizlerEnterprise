use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::services::auth::AuthProvider;

use super::error::HttpErrorResponse;

pub type AuthToken = String;

#[derive(Debug, Error)]
pub enum OIDError {}

impl HttpErrorResponse for OIDError {}

/// Request to check an OpenID token with the provider
#[derive(Deserialize)]
pub struct OIDConfirmRequest {
    /// The provider the token is from
    pub provider: AuthProvider,
    /// The token itself
    pub token: String,
}

/// Response from handling an OpenID token with a specific provider
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum OIDConfirmResponse {
    /// The OpenID token was successfully confirmed
    Success {
        /// The default username based on the name present in the claim
        default_username: String,
    },

    /// An account already exists and is setup with the provided auth
    /// provider so a auth token is provided instead
    Existing {
        /// The auth token to log the user in with
        token: AuthToken,
    },

    /// There is an existing account but that account is
    /// not setup for the specific auth provider
    Conflict,
}
