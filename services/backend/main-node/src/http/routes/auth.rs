use std::sync::Arc;

use anyhow::anyhow;
use axum::{routing::post, Extension, Json, Router};
use openid::{DiscoveredClient, IdToken, StandardClaims};
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use tracing::error;

use crate::database::entities::user::{CreateUser, User};
use crate::database::entities::user_link::UserLink;
use crate::http::models::{auth::*, error::HttpResult};
use crate::services::auth::AuthService;
use crate::utils::hashing::hash_password;

/// Defines the routes under the route group of /auth
pub fn routes() -> Router {
    Router::new()
        // OpenID routes
        .nest(
            "/oid",
            Router::new()
                // Confirm OpenID token
                .route("/confirm", post(openid_confirm)),
        )
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
        let link = UserLink::find_by_user(&db, &existing, req.provider).await?;

        // Error if the accounts aren't linked
        if link.is_none() {
            return Err(OIDError::NotLinked.into());
        }

        // Create an auth token
        let token = auth.create_user_token(&existing);

        return Ok(Json(OIDConfirmResponse::Existing { token }));
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
) -> HttpResult<Json<OIDCreateResponse>> {
    let client = auth
        .get_provider(req.provider)
        .await
        .ok_or(OIDError::ProviderUnavailable)?;

    // Decode the token claim
    let claims = decode_openid_token(&client, req.token)?;

    // Obtain the email address from user info
    let email = claims.userinfo.email.ok_or(OIDError::ClaimMissingEmail)?;

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
                let user = User::create(
                    db,
                    CreateUser {
                        email,
                        username: req.username,
                        password: hashed_password,
                    },
                )
                .await?;

                // Create a link for the provider to the user
                _ = UserLink::create(db, &user, req.provider).await?;

                Ok::<_, DbErr>(user)
            })
        })
        .await?;

    let token = auth.create_user_token(&user);

    Ok(Json(OIDCreateResponse { token }))
}
