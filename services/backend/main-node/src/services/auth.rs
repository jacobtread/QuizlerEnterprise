//! This service is responsible for authentication with external services such as
//! Google, Microsoft, ..etc

use crate::database::entities::{user::User, user_refresh_token::UserRefreshToken};
use anyhow::Context;
use chrono::{Duration, Utc};
use futures::{stream::FuturesUnordered, StreamExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use moka::future::Cache;
use openid::DiscoveredClient;
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
    SeedableRng,
};
use sea_orm::{ConnectionTrait, DbErr, DeriveActiveEnum};
use serde::{Deserialize, Serialize};
use std::{ops::Add, sync::Arc};
use strum::Display;
use thiserror::Error;
use tracing::{debug, error};

pub struct AuthService {
    providers: Cache<AuthProvider, Arc<DiscoveredClient>>,

    /// Header for JWT tokens
    jwt_header: Header,
    /// Validation for JWT tokens
    jwt_validation: Validation,
    /// Key for encoding JWT tokens
    encoding_key: EncodingKey,
    /// Key for decoding JWT tokens
    decoding_key: DecodingKey,
}

/// Provider for authentication
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    sea_orm::EnumIter,
    Serialize,
    Deserialize,
    DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum AuthProvider {
    #[sea_orm(string_value = "GOOGLE")]
    Google,
    #[sea_orm(string_value = "MICROSOFT")]
    Microsoft,
}

impl AuthProvider {
    /// Environment variable prefix for this providers keys
    pub fn env_prefix(&self) -> &'static str {
        match self {
            AuthProvider::Google => "GOOGLE_OPENID",
            AuthProvider::Microsoft => "MICROSOFT_OPENID",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    /// ID of the user this claim represents
    #[serde(rename = "sub")]
    pub user_id: u32,
    /// Expiry time UTC timestamp
    exp: i64,
}

const API_JWT_TOKEN_KEY: &str = "API_JWT_TOKEN_KEY";

#[derive(Serialize)]
pub struct UserTokenData {
    /// The token itself
    pub token: String,
    /// The refresh token for refreshing this token
    pub refresh_token: String,
    /// UTC timestamp for when the token expires
    pub expiry: i64,
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error(transparent)]
    Database(#[from] DbErr),
    #[error("Invalid refresh token")]
    InvalidRefreshToken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Failed to create token")]
    CreateToken(#[from] jsonwebtoken::errors::Error),
}

impl AuthService {
    /// Tokens are short lived 30min tokens that get refreshed
    const USER_TOKEN_EXPIRY_MINUTES: i64 = 30;
    /// Length of refresh tokens
    const REFRESH_TOKEN_LENGTH: usize = 128;

    /// Creates the authentication service and initializes the
    /// providers in the background
    pub fn new() -> Arc<Self> {
        let providers = Cache::builder()
            // Refresh providers every 24 hours
            .time_to_live(std::time::Duration::from_secs(60 * 60 * 24))
            .initial_capacity(2)
            .build();

        // Create JWT keys and header
        let key = std::env::var(API_JWT_TOKEN_KEY).expect("Missing JWT token key");
        let key_bytes = key.as_bytes();
        let encoding_key = EncodingKey::from_secret(key_bytes);
        let decoding_key = DecodingKey::from_secret(key_bytes);
        let jwt_header = Header::new(Algorithm::HS256);
        let jwt_validation = Validation::new(Algorithm::HS256);

        let service = Arc::new(Self {
            providers,
            encoding_key,
            decoding_key,
            jwt_header,
            jwt_validation,
        });

        let init_service = service.clone();

        // Initialize the providers in a separate task
        tokio::spawn(async move {
            init_service.initialize_providers().await;
        });

        service
    }

    /// Creates a JWT token for the user
    pub async fn create_user_token<C>(
        &self,
        db: &C,
        user: &User,
    ) -> Result<UserTokenData, TokenError>
    where
        C: ConnectionTrait,
    {
        let expiry = Utc::now()
            .add(Duration::minutes(Self::USER_TOKEN_EXPIRY_MINUTES))
            .timestamp();

        // Create the user token
        let token = jsonwebtoken::encode(
            &self.jwt_header,
            &UserClaims {
                user_id: user.id,
                exp: expiry,
            },
            &self.encoding_key,
        )?;

        // Create a refresh token
        let refresh_token = Self::create_refresh_token(db, user).await?;

        Ok(UserTokenData {
            token,
            refresh_token,
            expiry,
        })
    }

    /// Refreshes a user token using the provided `refresh_token`
    pub async fn refresh_user_token<C>(
        &self,
        db: &C,
        refresh_token: &str,
    ) -> Result<UserTokenData, TokenError>
    where
        C: ConnectionTrait,
    {
        // Find the token data
        let token = UserRefreshToken::find_by_token(db, refresh_token)
            .await?
            .ok_or(TokenError::InvalidRefreshToken)?;

        // Find the user to refresh the token for
        let user_id = token.user_id;
        let user = User::find_by_id(db, user_id)
            .await?
            .ok_or(TokenError::InvalidRefreshToken)?;

        // Create the new token and refresh token
        self.create_user_token(db, &user).await
    }

    /// Creates a unique refresh token for the provided `user`
    async fn create_refresh_token<C>(db: &C, user: &User) -> Result<String, TokenError>
    where
        C: ConnectionTrait,
    {
        let mut rng = StdRng::from_entropy();

        loop {
            let token = Alphanumeric.sample_string(&mut rng, Self::REFRESH_TOKEN_LENGTH);

            // Check the token isn't already in use
            if UserRefreshToken::find_by_token(db, &token).await?.is_none() {
                // Create the token
                let token = UserRefreshToken::create(db, user, token).await?;

                return Ok(token.refresh_token);
            }
        }
    }

    /// Verifies the provided user token returning the associated user
    pub fn verify_user_token(&self, token: &str) -> Result<UserClaims, TokenError> {
        let token_data: jsonwebtoken::TokenData<UserClaims> =
            decode(token, &self.decoding_key, &self.jwt_validation)
                .map_err(|_| TokenError::InvalidToken)?;
        Ok(token_data.claims)
    }

    /// Initializes all the auth providers in this service
    pub async fn initialize_providers(&self) {
        let mut futures = [AuthProvider::Google, AuthProvider::Microsoft]
            .into_iter()
            .map(|provider| self.get_provider(provider))
            .collect::<FuturesUnordered<_>>();

        while futures.next().await.is_some() {}
    }

    /// Attempts to get the specified provider from the cache, will
    /// initialize the provider if it is expired or not initialized
    pub async fn get_provider(&self, provider: AuthProvider) -> Option<Arc<DiscoveredClient>> {
        match self
            .providers
            .try_get_with(provider, Self::create_provider(provider))
            .await
        {
            Ok(value) => Some(value),
            Err(err) => {
                error!(name: "err_initialize_provider", error = %err, "Failed to initialize auth provider");
                None
            }
        }
    }

    /// Attempts to create a new provider for the provided [AuthProvider] type
    #[tracing::instrument]
    pub async fn create_provider(provider: AuthProvider) -> anyhow::Result<Arc<DiscoveredClient>> {
        let env_prefix = provider.env_prefix();

        let issuer = std::env::var(format!("{env_prefix}_ISSUER"))
            .with_context(|| format!("Missing {env_prefix}_ISSUER for {provider}"))?;
        let issuer = reqwest::Url::parse(&issuer)
            .with_context(|| format!("Missing invalid issuer URL for {provider}"))?;

        let client_id = std::env::var(format!("{env_prefix}_CLIENT_ID"))
            .with_context(|| format!("Missing {env_prefix}_CLIENT_ID for {provider}"))?;
        let client_secret = std::env::var(format!("{env_prefix}_CLIENT_SECRET"))
            .with_context(|| format!("Missing {env_prefix}_CLIENT_SECRET for {provider}"))?;

        let client = DiscoveredClient::discover(client_id, client_secret, None, issuer)
            .await
            .with_context(|| format!("Failed to initialize OpenID client for {provider}"))?;

        debug!(name: "start_auth_provider", %provider, "Started auth provider");

        Ok(Arc::new(client))
    }
}
