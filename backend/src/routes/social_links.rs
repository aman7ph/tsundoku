use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::social_links};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(
            social_links::list_social_links,
            social_links::create_social_link
        ))
        .routes(routes!(
            social_links::update_social_link,
            social_links::delete_social_link
        ))
}
