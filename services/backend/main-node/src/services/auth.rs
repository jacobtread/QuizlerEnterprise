//! This service is responsible for authentication with external services such as
//! Google, Microsoft, ..etc

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use futures::{stream::FuturesUnordered, StreamExt};
use moka::future::Cache;
use openid::DiscoveredClient;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use tracing::{debug, error};

pub struct AuthService {
    providers: Cache<AuthProvider, Arc<DiscoveredClient>>,
}

/// Provider for authentication
#[derive(Debug, Display, Clone, Copy, Hash, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum AuthProvider {
    Google,
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

impl AuthService {
    /// Creates the authentication service and initializes the
    /// providers in the background
    pub fn new() -> Arc<Self> {
        let providers = Cache::builder()
            // Refresh providers every 24 hours
            .time_to_live(Duration::from_secs(60 * 60 * 24))
            .initial_capacity(2)
            .build();

        let service = Arc::new(Self { providers });

        let init_service = service.clone();

        // Initialize the providers in a separate task
        tokio::spawn(async move {
            init_service.initialize_providers().await;
        });

        service
    }

    /// Initializes all the auth providers in this service
    pub async fn initialize_providers(&self) {
        let mut futures = AuthProvider::iter()
            .map(|provider| self.get_provider(provider))
            .collect::<FuturesUnordered<_>>();

        while let Some(future) = futures.next().await {
            if let Err(err) = future {
                error!(name: "err_initialize_provider", error = %err);
            }
        }
    }

    /// Attempts to get the specified provider from the cache, will
    /// initialize the provider if it is expired or not initialized
    pub async fn get_provider(
        &self,
        provider: AuthProvider,
    ) -> Result<Arc<DiscoveredClient>, Arc<anyhow::Error>> {
        self.providers
            .try_get_with(provider, Self::create_provider(provider))
            .await
            .clone()
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
