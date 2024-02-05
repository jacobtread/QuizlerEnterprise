use crate::http::models::error::{HttpError, HttpErrorResponse};
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error};

/// Protects the route with a reCaptcha checker that requires a recaptcha
/// header is set with a valid recaptcha token
pub struct ProtectReCaptcha;

/// Header to extract the reCaptcha token from
pub const RECAPTCHA_HEADER: &str = "x-captcha-token";
/// Google API endpoint for reCaptcha validation
const VERIFY_ENDPOINT: &str = "https://www.google.com/recaptcha/api/siteverify";

#[derive(Debug, Error)]
pub enum CaptchaError {
    /// Token wasn't provided
    #[error("Missing reCaptcha token")]
    MissingToken,
    /// Token was invalid
    #[error("Invalid reCaptcha token")]
    InvalidToken,
    /// User failed the captcha test
    #[error("Failed reCaptcha validation")]
    Failed,
    /// Failed to make the validate request
    #[error("Failed to request captcha validation")]
    Request,
    /// Failed when parsing the validate response
    #[error("Failed to parse captcha response")]
    Parsing,
}

impl HttpError for CaptchaError {}

/// Request for Google to verify a captcha request
#[derive(Serialize)]
struct CaptchaRequest<'a> {
    /// Client captcha response token
    response: &'a str,
    /// Server reCaptcha secret
    secret: String,
}

/// Response to a Google captcha verification
#[derive(Deserialize)]
struct CaptchaResponse {
    /// Determines whether the captcha was a success
    success: bool,
}

#[async_trait]
impl<S> FromRequestParts<S> for ProtectReCaptcha
where
    S: Send + Sync,
{
    type Rejection = HttpErrorResponse;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let server_secret = match std::env::var("RECAPTCHA_SECRET_KEY") {
            Ok(value) => value,
            Err(_) => {
                debug!(name: "captcha", "Server missing RECAPTCHA_SECRET_KEY, skipping captcha validation");
                return Ok(Self);
            }
        };

        // Automatically succeed captchas in debug mode
        // if cfg!(debug_assertions) {
        //     return Ok(Self);
        // }

        // Ensure that the header is present
        let header = parts
            .headers
            .get(RECAPTCHA_HEADER)
            .ok_or(CaptchaError::MissingToken)?;

        // Parse the header token
        let token = header.to_str().map_err(|_| CaptchaError::InvalidToken)?;

        let client: reqwest::Client = reqwest::Client::new();

        // Request the verify endpoint
        let response: reqwest::Response = client
            .post(VERIFY_ENDPOINT)
            .form(&CaptchaRequest {
                response: token,
                secret: server_secret,
            })
            .send()
            .await
            .map_err(|_| CaptchaError::Request)?;

        // Parse the JSON response
        let response: CaptchaResponse = response.json().await.map_err(|_| CaptchaError::Parsing)?;

        debug!(name: "captcha_success", %token, "Captcha completed successfully");

        if response.success {
            Ok(Self)
        } else {
            Err(CaptchaError::Failed.into())
        }
    }
}
