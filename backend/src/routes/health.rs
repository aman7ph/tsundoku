use axum::{routing::get, Router};

use crate::{handlers::health, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(health::hello))
        .route("/health/db", get(health::health_db))
}