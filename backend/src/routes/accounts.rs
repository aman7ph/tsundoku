use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, handlers::accounts};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(accounts::list_accounts, accounts::create_account))
        .routes(routes!(accounts::update_account, accounts::delete_account))
}
