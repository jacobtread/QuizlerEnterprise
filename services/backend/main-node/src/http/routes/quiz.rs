use crate::database::entities::quiz::{Quiz, QuizId};
use crate::database::entities::user::User;
use crate::http::middleware::auth::Auth;
use crate::http::middleware::json::ValidJson;
use crate::http::models::error::HttpResult;
use crate::http::models::quiz::{CreateQuizRequest, QuizError};
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use sea_orm::DatabaseConnection;

/// Defines the routes under the route group of /user
pub fn routes() -> Router {
    Router::new()
        // Self route
        .route("/create", post(create_quiz))
        .nest("/:id", Router::new().route("/", get(get_quiz)))
}

/// POST /quiz/create
///
/// Requests the creation of a new quiz
async fn create_quiz(
    Auth(user): Auth,
    Extension(db): Extension<DatabaseConnection>,
    ValidJson(req): ValidJson<CreateQuizRequest>,
) -> HttpResult<Json<Quiz>> {
    // Create the new quiz
    let quiz = Quiz::create(&db, &user, req.title).await?;

    Ok(Json(quiz))
}
/// GET /quiz/:id
///
/// Requests the details of a quiz
async fn get_quiz(
    Auth(user): Auth,
    Path(id): Path<QuizId>,
    Extension(db): Extension<DatabaseConnection>,
) -> HttpResult<Json<Quiz>> {
    // Create the new quiz
    let quiz = Quiz::find_by_id(&db, id)
        .await?
        .ok_or(QuizError::NotFound)?;

    if !quiz.owner.eq(&user.id) {
        return Err(QuizError::MissingPermission.into());
    }

    Ok(Json(quiz))
}
