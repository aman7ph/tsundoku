use axum::{extract::State, http::StatusCode};

use crate::config::AppState;

#[utoipa::path(
    get,
    path = "/",
    tag = "Health",
    responses(
        (status = 200, description = "Service is running", body = String, content_type = "text/plain")
    )
)]
pub async fn hello() -> &'static str {
    "Hello from Tsundoku! 積ん読"
}

#[utoipa::path(
    get,
    path = "/health/db",
    tag = "Health",
    responses(
        (status = 200, description = "Database is reachable", body = String, content_type = "text/plain"),
        (status = 500, description = "Database is not reachable")
    )
)]
pub async fn health_db(State(state): State<AppState>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("Database connection OK")
}
