use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use http::init_router;
use std::error::Error;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
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

    let authentication = services::auth::AuthService::new();

    let app = init_router().layer(Extension(authentication));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
