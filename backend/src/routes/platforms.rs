use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::platforms};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(
            platforms::list_platforms,
            platforms::create_platform
        ))
        .routes(routes!(
            platforms::update_platform,
            platforms::delete_platform
        ))
}
