use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{config::AppState, docs::ApiDoc};

mod accounts;
mod categories;
mod health;
mod platforms;
mod resources;
mod sections;

pub fn create_router(state: AppState) -> Router {
    // Each feature's router registers its handlers; the OpenAPI spec is built from them
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(health::router())
        .merge(sections::router())
        .merge(categories::router())
        .merge(resources::router())
        .merge(platforms::router())
        .merge(accounts::router())
        .split_for_parts();

    router
        .merge(Scalar::with_url("/docs", api))
        .with_state(state)
}
