use crate::database::entities::user::User;
use crate::http::middleware::auth::Auth;
use axum::routing::get;
use axum::{Json, Router};

/// Defines the routes under the route group of /user
pub fn routes() -> Router {
    Router::new()
        // Self route
        .route("/self", get(get_active_user))
}

/// GET /user/self
///
/// Requests the current authenticated user details
async fn get_active_user(Auth(user): Auth) -> Json<User> {
    Json(user)
}
