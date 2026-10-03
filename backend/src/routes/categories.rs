use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::categories};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(
            categories::list_categories,
            categories::create_category
        ))
        .routes(routes!(
            categories::update_category,
            categories::delete_category
        ))
        .routes(routes!(categories::move_category))
}
