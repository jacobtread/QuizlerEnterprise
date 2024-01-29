use std::sync::Arc;

use axum::{routing::post, Extension, Json, Router};

use crate::http::models::{auth::*, error::HttpResult};
use crate::services::auth::AuthService;

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

/// POST /auth/oid/confirm
///
/// Confirms an OpenID token by verifying the token claims
async fn openid_confirm(
    Extension(auth): Extension<Arc<AuthService>>,
    Json(req): Json<OIDConfirmRequest>,
) -> HttpResult<Json<OIDConfirmResponse>> {
    let client = auth.get_provider(req.provider).await;

    Ok(Json(OIDConfirmResponse::Conflict))
}
