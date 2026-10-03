use axum::Router;

use crate::state::AppState;

mod health;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .with_state(state)
}