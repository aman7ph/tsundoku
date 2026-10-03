use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::social_categories};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(
            social_categories::list_social_categories,
            social_categories::create_social_category
        ))
        .routes(routes!(
            social_categories::update_social_category,
            social_categories::delete_social_category
        ))
}
