use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::sections};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(sections::list_sections, sections::create_section))
        .routes(routes!(sections::update_section, sections::delete_section))
}
