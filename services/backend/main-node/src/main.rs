use anyhow::Context;
use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use http::init_router;
use sea_orm::DatabaseConnection;
use services::auth::AuthService;
use std::{error::Error, sync::Arc};
use tracing::{info, Level};

pub mod database;
pub mod http;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load environment variables from .env file
    dotenv()?;

    utils::tracing::init_tracing()?;

    let authentication: Arc<AuthService> = services::auth::AuthService::new();
    let db: DatabaseConnection = database::connect()
        .await
        .context("Connecting to database")?;

    let app = init_router()
        .layer(Extension(db))
        .layer(Extension(authentication));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("Binding server listener")?;
    axum::serve(listener, app)
        .await
        .context("Serving application")?;

    Ok(())
}
