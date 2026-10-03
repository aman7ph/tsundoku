use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::{config::AppState, utils::error::AppError};

// Any handler that takes `CurrentUser` as a parameter requires a signed-in user
pub struct CurrentUser(pub Uuid);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // DEV ONLY: replaced by real JWT verification in the auth step
        let id = parts
            .headers
            .get("x-user-id")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or(AppError::Unauthorized)?;

        Ok(CurrentUser(id))
    }
}
