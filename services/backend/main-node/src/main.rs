use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::error::Error;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};

pub mod database;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load environment variables from .env file
    dotenv()?;

    utils::tracing::init_tracing()?;

    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                info!(name: "completed", "Served hello world");
                "Hello, World!"
            }),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
