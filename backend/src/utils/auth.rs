use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::{
    config::AppState,
    utils::{error::AppError, jwt},
};

// Any handler that takes `CurrentUser` as a parameter requires a signed-in user
pub struct CurrentUser(pub Uuid);

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(AppError::Unauthorized)?;

        let user_id = jwt::verify_access_token(&state.auth.jwt_secret, token)?;
        Ok(CurrentUser(user_id))
    }
}