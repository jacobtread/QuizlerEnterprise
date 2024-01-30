//! This service is responsible for authentication with external services such as
//! Google, Microsoft, ..etc

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use futures::{stream::FuturesUnordered, StreamExt};
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use moka::future::Cache;
use openid::DiscoveredClient;
use sea_orm::DeriveActiveEnum;
use serde::{Deserialize, Serialize};
use strum::Display;
use tracing::{debug, error};

use crate::database::entities::user::User;

pub struct AuthService {
    providers: Cache<AuthProvider, Arc<DiscoveredClient>>,

    /// Header for jwt tokens
    jwt_header: Header,
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
    exp: usize,
}

const API_JWT_TOKEN_KEY: &str = "API_JWT_TOKEN_KEY";

impl AuthService {
    /// Creates the authentication service and initializes the
    /// providers in the background
    pub fn new() -> Arc<Self> {
        let providers = Cache::builder()
            // Refresh providers every 24 hours
            .time_to_live(Duration::from_secs(60 * 60 * 24))
            .initial_capacity(2)
            .build();

        // Create JWT keys and header
        let key = std::env::var(API_JWT_TOKEN_KEY).expect("Missing JWT token key");
        let key_bytes = key.as_bytes();
        let encoding_key = EncodingKey::from_secret(key_bytes);
        let decoding_key = DecodingKey::from_secret(key_bytes);
        let jwt_header = Header::new(jsonwebtoken::Algorithm::HS256);

        let service = Arc::new(Self {
            providers,
            encoding_key,
            decoding_key,
            jwt_header,
        });

        let init_service = service.clone();

        // Initialize the providers in a separate task
        tokio::spawn(async move {
            init_service.initialize_providers().await;
        });

        service
    }

    /// Creates a JWT token for the user
    pub fn create_user_token(&self, user: &User) -> String {
        jsonwebtoken::encode(&self.jwt_header, &UserClaims {
            user_id: user.id,
        }, key)

        todo!("Create user token")
    }

    pub fn verify_user_token(&self, token: &str) {
        todo!("Verify user token")
    }

    /// Initializes all the auth providers in this service
    pub async fn initialize_providers(&self) {
        let mut futures = [AuthProvider::Google, AuthProvider::Microsoft]
            .into_iter()
            .map(|provider| self.get_provider(provider))
            .collect::<FuturesUnordered<_>>();

        while let Some(_) = futures.next().await {}
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
                return None;
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
