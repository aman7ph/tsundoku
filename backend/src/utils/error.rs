use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

pub enum AppError {
    Unauthorized,
    NotFound,
    BadRequest(String),
    Internal,
}
/// Shape of every error response
#[derive(Serialize, ToSchema)]
pub struct ErrorBody {
    #[schema(example = "name cannot be empty")]
    pub error: String,
}

// Lets `?` turn a database error into an AppError automatically
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        eprintln!("database error: {err}");
        AppError::Internal
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };

        (status, Json(ErrorBody { error: message })).into_response()
    }
}
