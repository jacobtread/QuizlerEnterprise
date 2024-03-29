use crate::database::entities::user::{CreateUser, User};
use crate::database::entities::user_link::UserLink;
use crate::http::middleware::json::{ExtractJson, ValidJson};
use crate::http::middleware::recaptcha::ProtectReCaptcha;
use crate::http::models::{auth::*, error::HttpResult};
use crate::services::auth::{AuthProvider, AuthService};
use crate::utils::assert::assert;
use crate::utils::hashing::{hash_password, verify_password};
use crate::utils::types::{EmailAddress, Username};
use anyhow::Context;
use axum::routing::get;
use axum::{routing::post, Extension, Json, Router};
use openid::{DiscoveredClient, IdToken, StandardClaims, Token};
use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};
use std::sync::Arc;
use tracing::error;

/// Defines the routes under the route group of /auth
pub fn routes() -> Router {
    Router::new()
        .nest(
            "/basic",
            Router::new()
                .route("/register", post(basic_register))
                .route("/login", post(basic_login)),
        )
        // OpenID routes
        .nest(
            "/oid",
            Router::new()
                // View available providers
                .route("/providers", get(openid_providers))
                // Authenticate OpenID code
                .route("/authenticate", post(openid_authenticate))
                // Confirm OpenID token
                .route("/create", post(openid_create)),
        )
        // Token routes
        .nest(
            "/token",
            Router::new().route("/refresh", post(refresh_token)),
        )
}

/// GET /auth/basic/register
///
/// Request to register an account using basic email, username, password
/// credentials
async fn basic_register(
    _: ProtectReCaptcha,
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    ValidJson(req): ValidJson<BasicRegisterRequest>,
) -> HttpResult<Json<TokenResponse>> {
    // Ensure the email address isn't already in use
    assert(
        !User::is_email_taken(&db, &req.email).await?,
        AuthError::EmailExists,
    )?;

    // Ensure the username isn't already in use
    assert(
        !User::is_username_taken(&db, &req.username).await?,
        AuthError::UsernameExists,
    )?;

    let hashed_password: String =
        hash_password(req.password.as_str()).context("Hashing password")?;

    // Create the new user
    let user = User::create(
        &db,
        CreateUser {
            email: req.email.into_inner(),
            username: req.username.into_inner(),
            password: hashed_password,
        },
    )
    .await?;

    // Create an auth token
    let user_token_data = auth.create_user_token(&db, &user).await.map_err(|error| {
        error!(name: "err_issue_token", %error, "Failed to issue user token");
        AuthError::FailedTokenIssue
    })?;

    Ok(Json(TokenResponse { user_token_data }))
}

async fn basic_login(
    _: ProtectReCaptcha,
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    ValidJson(req): ValidJson<BasicLoginRequest>,
) -> HttpResult<Json<TokenResponse>> {
    let user = User::find_by_email(&db, &req.email)
        .await?
        .ok_or(AuthError::EmailNotFound)?;

    verify_password(req.password.as_str(), &user.password)
        .map_err(|_| AuthError::IncorrectPassword)?;

    // Create an auth token
    let user_token_data = auth.create_user_token(&db, &user).await.map_err(|error| {
        error!(name: "err_issue_token", %error, "Failed to issue user token");
        AuthError::FailedTokenIssue
    })?;

    Ok(Json(TokenResponse { user_token_data }))
}

/// GET /auth/oid/providers
///
/// Requests a collection of OpenID providers and their associated
/// auth URL
async fn openid_providers(
    Extension(auth): Extension<Arc<AuthService>>,
) -> HttpResult<Json<OIDProvidersResponse>> {
    // Load the available providers
    let providers = auth.get_all_providers().await;

    let providers: Vec<(AuthProvider, OIDProvider)> = providers
        .into_iter()
        // Remove any providers with un-initialized clients
        .filter_map(|(provider, client)| client.map(|client| (provider, client)))
        // Create the OIDProvider
        .map(|(provider, client)| {
            // Create an auth URL for the provider
            let auth_url = client.auth_url(&openid::Options {
                scope: Some(AuthProvider::SCOPES.to_string()),
                state: Some(provider.to_string()),
                ..Default::default()
            });

            (provider, OIDProvider { auth_url })
        })
        .collect();

    Ok(Json(OIDProvidersResponse { providers }))
}

/// POST /auth/oid/authenticate
///
/// Requests an OpenID token from an OpenID code for a specific
/// provider
async fn openid_authenticate(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    ExtractJson(req): ExtractJson<OIDAuthenticateRequest>,
) -> HttpResult<Json<OIDAuthenticateResponse>> {
    let client = auth
        .get_provider(req.provider)
        .await
        .ok_or(OIDError::ProviderUnavailable)?;

    // Exchange the code for a token
    let token: Token = client
        .request_token(&req.code)
        .await
        .map_err(|_| OIDError::Authentication)?
        .into();

    let token: IdToken<StandardClaims> = token.id_token.ok_or(OIDError::Authentication)?;

    // Decode the token claim
    let claims: StandardClaims = decode_openid_token(&client, token.clone())?;

    // Obtain the email address from user info
    let email: EmailAddress = claims
        .userinfo
        .email
        .ok_or(OIDError::ClaimMissingEmail)?
        .parse()?;

    // Obtain the default username if one is present
    let default_username: Option<Username> = claims
        .userinfo
        .preferred_username
        .and_then(|value| value.parse().ok());

    let existing = User::find_by_email(&db, &email).await?;

    if let Some(existing) = existing {
        // Find an existing link
        let _ = UserLink::find_by_user(&db, &existing, req.provider)
            .await?
            .ok_or(OIDError::NotLinked)?;

        // Create an auth token
        let user_token_data = auth
            .create_user_token(&db, &existing)
            .await
            .map_err(|err| {
                error!(name: "err_issue_token", error = %err, "Failed to issue user token");
                AuthError::FailedTokenIssue
            })?;

        Ok(Json(OIDAuthenticateResponse::ExistingLinked(
            TokenResponse { user_token_data },
        )))
    } else {
        Ok(Json(OIDAuthenticateResponse::CreateAccount {
            token: Box::new(token),
            default_username,
        }))
    }
}

/// POST /auth/token/refresh
///
/// Requests a refresh of a token using a provided refresh token
async fn refresh_token(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    ExtractJson(req): ExtractJson<RefreshTokenRequest>,
) -> HttpResult<Json<TokenResponse>> {
    let user_token_data = auth
        .refresh_user_token(&db, &req.refresh_token)
        .await
        .map_err(|err| {
            error!(name: "err_refresh_token", error = %err, "Failed to refresh user token");
            AuthError::FailedTokenIssue
        })?;

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
            OIDError::InvalidToken
        })?;

    // Validate the token
    client
        .validate_token(&token, None, None)
        // Handle invalid token error
        .map_err(|err| {
            error!(name: "openid_validate_token", error = %err, "Token failed validation");
            OIDError::InvalidToken
        })?;

    // Extract the token claims
    let (_, claims) = token.unwrap_decoded();

    Ok(claims)
}

/// POST /auth/oid/create
///
/// Creates an account from an OpenID token and user provided details
async fn openid_create(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(db): Extension<DatabaseConnection>,
    ValidJson(req): ValidJson<OIDCreateRequest>,
) -> HttpResult<Json<TokenResponse>> {
    let client = auth
        .get_provider(req.provider)
        .await
        .ok_or(OIDError::ProviderUnavailable)?;

    // Decode the token claim
    let claims = decode_openid_token(&client, req.token)?;

    // Obtain the email address from user info
    let email: EmailAddress = claims
        .userinfo
        .email
        .ok_or(OIDError::ClaimMissingEmail)?
        .parse()?;

    // Check if they've verified the email
    let email_verified = claims.userinfo.email_verified;

    // Ensure the email address isn't already in use
    assert(
        !User::is_email_taken(&db, &email).await?,
        AuthError::EmailExists,
    )?;

    // Ensure the username isn't already in use
    assert(
        !User::is_username_taken(&db, &req.username).await?,
        AuthError::UsernameExists,
    )?;

    let hashed_password: String =
        hash_password(req.password.as_str()).context("Hashing password")?;

    let user: User = db
        .transaction(move |db| {
            Box::pin(async move {
                // Create the new user
                let mut user = User::create(
                    db,
                    CreateUser {
                        email: email.into_inner(),
                        username: req.username.into_inner(),
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

    let user_token_data = auth
        .create_user_token(&db, &user)
        .await
        // Handle errors issuing
        .map_err(|err| {
            error!(name: "err_issue_token", error = %err, "Failed to issue user token");
            AuthError::FailedTokenIssue
        })?;

    Ok(Json(TokenResponse { user_token_data }))
}
