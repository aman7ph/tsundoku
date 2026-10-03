use axum::{extract::State, http::StatusCode};

use crate::state::AppState;

pub async fn hello() -> &'static str {
    "Hello from Tsundoku! 積ん読"
}

pub async fn health_db(State(state): State<AppState>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("Database connection OK")
}