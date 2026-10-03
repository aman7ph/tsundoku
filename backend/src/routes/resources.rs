use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::resources};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(
            resources::list_resources,
            resources::create_resource
        ))
        .routes(routes!(
            resources::update_resource,
            resources::delete_resource
        ))
}
