use axum::{
    http::{header, HeaderName, HeaderValue, Method},
    Router,
};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

use super::middleware::recaptcha::RECAPTCHA_HEADER;

mod auth;
mod quiz;
mod user;

/// Initializes the router and all routes in the app
pub fn init_router() -> Router {
    let hub_url = std::env::var("HUB_BASE_URL")
        .expect("Missing HUB_BASE_URL")
        .parse::<HeaderValue>()
        .expect("Invalid HUB_BASE_URL");

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            HeaderName::from_static(RECAPTCHA_HEADER),
        ])
        .allow_credentials(true)
        .allow_origin(hub_url);

    Router::new()
        .nest("/auth", auth::routes())
        .nest("/user", user::routes())
        .nest("/quiz", quiz::routes())
        // Request tracing
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros),
                ),
        )
        .layer(cors)
}
