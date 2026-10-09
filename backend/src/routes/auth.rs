use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::auth};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(auth::google_login))
        .routes(routes!(auth::refresh))
        .routes(routes!(auth::logout))
        .routes(routes!(auth::dev_login))
        .routes(routes!(auth::me))
}
