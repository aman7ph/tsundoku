use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::health};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(health::hello))
        .routes(routes!(health::health_db))
}
