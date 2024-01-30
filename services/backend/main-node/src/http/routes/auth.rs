use crate::database::entities::user::{CreateUser, User};
use crate::database::entities::user_link::UserLink;
use crate::http::models::{auth::*, error::HttpResult};
use crate::services::auth::AuthService;
use crate::utils::hashing::hash_password;
use anyhow::anyhow;
use axum::{routing::post, Extension, Json, Router};
use openid::{DiscoveredClient, IdToken, StandardClaims};
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use std::sync::Arc;
use tracing::error;

/// Defines the routes under the route group of /auth
pub fn routes() -> Router {
    Router::new()
        // OpenID routes
        .nest(
            "/oid",
            Router::new()
                // Confirm OpenID token
                .route("/confirm", post(openid_confirm))
                .route("/create", post(openid_create))
                .route("/login", post(openid_login)),
        )
        // Token routes
        .nest(
            "/token",
            Router::new().route("/refresh", post(refresh_token)),
        )
}

/// POST /auth/token/refresh
///
/// Requests a refresh of a token using a provided refresh token
async fn refresh_token(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    Json(req): Json<RefreshTokenRequest>,
) -> HttpResult<Json<TokenResponse>> {
    let user_token_data = match auth.refresh_user_token(&db, &req.refresh_token).await {
        Ok(value) => value,
        Err(err) => {
            error!(name: "err_refresh_token", error = %err, "Failed to refresh user token");

            return Err(AuthError::FailedTokenIssue.into());
        }
    };

    Ok(Json(TokenResponse { user_token_data }))
}

/// Decodes the provided `token` returning either the claims present
/// in the token or an error
fn decode_openid_token(
    client: &DiscoveredClient,
    mut token: IdToken<StandardClaims>,
) -> Result<StandardClaims, OIDError> {
    // Decode the token
    client
        .decode_token(&mut token)
        // Handle token error
        .map_err(|err| {
            error!(name: "openid_decode_token", error = %err, "Failed to decode token");
            OIDError::Token
        })?;

    // Extract the token claims
    let (_, claims) = token.unwrap_decoded();

    Ok(claims)
}

/// POST /auth/oid/confirm
///
/// Confirms an OpenID token by verifying the token claims
async fn openid_confirm(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    Json(req): Json<OIDConfirmRequest>,
) -> HttpResult<Json<OIDConfirmResponse>> {
    let client = auth
        .get_provider(req.provider)
        .await
        .ok_or(OIDError::ProviderUnavailable)?;

    // Decode the token claim
    let claims = decode_openid_token(&client, req.token)?;

    // Obtain the email address from user info
    let email = claims.userinfo.email.ok_or(OIDError::ClaimMissingEmail)?;

    // Obtain the username if one is present
    let username = claims.userinfo.preferred_username;

    let existing = User::find_by_email(&db, &email).await?;

    if let Some(existing) = existing {
        // Find an existing link
        let _ = UserLink::find_by_user(&db, &existing, req.provider)
            .await?
            .ok_or(OIDError::NotLinked)?;

        return Ok(Json(OIDConfirmResponse::Existing));
    }

    Ok(Json(OIDConfirmResponse::Success {
        default_username: username,
    }))
}

/// POST /auth/oid/create
///
/// Creates an account from an OpenID token and user provided details
async fn openid_create(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    Json(req): Json<OIDCreateRequest>,
) -> HttpResult<Json<TokenResponse>> {
    let client = auth
        .get_provider(req.provider)
        .await
        .ok_or(OIDError::ProviderUnavailable)?;

    // Decode the token claim
    let claims = decode_openid_token(&client, req.token)?;

    // Obtain the email address from user info
    let email = claims.userinfo.email.ok_or(OIDError::ClaimMissingEmail)?;

    // Check if they've verified the email
    let email_verified = claims.userinfo.email_verified;

    // Ensure the user doesn't exist already
    if User::find_by_email(&db, &email).await?.is_some() {
        return Err(OIDError::AlreadyExists.into());
    }

    let hashed_password: String =
        hash_password(&req.password).map_err(|_| anyhow!("Failed to hash password"))?;

    let user: User = db
        .transaction(move |db| {
            Box::pin(async move {
                // Create the new user
                let mut user = User::create(
                    db,
                    CreateUser {
                        email,
                        username: req.username,
                        password: hashed_password,
                    },
                )
                .await?;

                // Verify the email if the provider says its verified
                if email_verified {
                    user = user.set_email_verified(db).await?;
                }

                // Create a link for the provider to the user
                _ = UserLink::create(db, &user, req.provider).await?;

                Ok::<_, DbErr>(user)
            })
        })
        .await?;

    let user_token_data = match auth.create_user_token(&db, &user).await {
        Ok(value) => value,
        Err(err) => {
            error!(name: "err_issue_token", error = %err, "Failed to issue user token");

            return Err(AuthError::FailedTokenIssue.into());
        }
    };

    Ok(Json(TokenResponse { user_token_data }))
}

/// POST /auth/oid/login
///
/// Logs into an account using an OpenID token
async fn openid_login(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    Json(req): Json<OIDConfirmRequest>,
) -> HttpResult<Json<TokenResponse>> {
    let client = auth
        .get_provider(req.provider)
        .await
        .ok_or(OIDError::ProviderUnavailable)?;

    // Decode the token claim
    let claims = decode_openid_token(&client, req.token)?;

    // Obtain the email address from user info
    let email = claims.userinfo.email.ok_or(OIDError::ClaimMissingEmail)?;

    // Find the user associated to the email
    let user = User::find_by_email(&db, &email)
        .await?
        .ok_or(OIDError::MissingAccount)?;

    // Find an existing link
    let _ = UserLink::find_by_user(&db, &user, req.provider)
        .await?
        .ok_or(OIDError::NotLinked)?;

    // Create an auth token
    let user_token_data = match auth.create_user_token(&db, &user).await {
        Ok(value) => value,
        Err(err) => {
            error!(name: "err_issue_token", error = %err, "Failed to issue user token");

            return Err(AuthError::FailedTokenIssue.into());
        }
    };

    Ok(Json(TokenResponse { user_token_data }))
}
